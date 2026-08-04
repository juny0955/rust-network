use crate::ipv4::Protocol::{ICMP, TCP, UDP};

#[derive(Debug, PartialEq, Eq)]
pub struct IPv4 {
    pub version: u8,
    pub ihl: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: Protocol,
    pub checksum: u16,
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub options: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IPv4Error {
    InvalidByteLength,
    InvalidHeader,
    InvalidChecksum,
    InvalidProtocol,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
}

impl Protocol {
    fn parse(byte: u8) -> Result<Protocol, IPv4Error> {
        match byte {
            1 => Ok(ICMP),
            6 => Ok(TCP),
            17 => Ok(UDP),
            _ => Err(IPv4Error::InvalidProtocol),
        }
    }
}

impl IPv4 {
    pub fn parse(bytes: &[u8]) -> Result<Self, IPv4Error> {
        if bytes.len() < 20 {
            eprintln!("IP Packet 바이트 길이가 짧습니다");
            return Err(IPv4Error::InvalidHeader);
        }

        let (version, ihl) = Self::parse_version_ihl(bytes[0])?;
        let header_len = usize::from(ihl) * 4;

        if bytes.len() < header_len {
            return Err(IPv4Error::InvalidByteLength);
        }

        Self::validate_checksum(&bytes[..header_len])?;

        let (dscp, ecn) = Self::parse_dscp_ecn(bytes[1]);
        let total_length = u16::from_be_bytes([bytes[2], bytes[3]]);
        let total_length_usize = total_length as usize;

        if total_length_usize < header_len || total_length_usize > bytes.len() {
            return Err(IPv4Error::InvalidHeader);
        }

        let identification = u16::from_be_bytes([bytes[4], bytes[5]]);
        let (flags, fragment_offset) = Self::parse_flags_frag_offset([bytes[6], bytes[7]]);
        let ttl = bytes[8];
        let protocol = Protocol::parse(bytes[9])?;
        let checksum = u16::from_be_bytes([bytes[10], bytes[11]]);
        let src_ip = [bytes[12], bytes[13], bytes[14], bytes[15]];
        let dst_ip = [bytes[16], bytes[17], bytes[18], bytes[19]];
        let options = bytes[20..header_len].to_vec();

        Ok(Self {
            version,
            ihl,
            dscp,
            ecn,
            total_length,
            identification,
            flags,
            fragment_offset,
            ttl,
            protocol,
            checksum,
            src_ip,
            dst_ip,
            options,
        })
    }

    /// version 상위 4비트
    /// ihl 하위 4비트
    fn parse_version_ihl(byte: u8) -> Result<(u8, u8), IPv4Error> {
        let version = byte >> 4;
        let ihl = byte & 0x0F;

        if version != 4 || ihl < 5 {
            return Err(IPv4Error::InvalidHeader);
        }

        Ok((version, ihl))
    }

    /// dscp 상위 6비트
    /// ecn 하위 2비트
    fn parse_dscp_ecn(byte: u8) -> (u8, u8) {
        let dscp = byte >> 2;
        let ecn = byte & 0b0000_0011;

        (dscp, ecn)
    }

    /// flags 상위 3비트
    /// fragment_offset 하위 13비트
    fn parse_flags_frag_offset(byte: [u8; 2]) -> (u8, u16) {
        let flags = byte[0] >> 5;
        let fragment_offset = (u16::from(byte[0] & 0b0001_1111) << 8) | u16::from(byte[1]);

        (flags, fragment_offset)
    }

    fn validate_checksum(header_bytes: &[u8]) -> Result<(), IPv4Error> {
        let mut sum: u32 = 0;
        for pair in header_bytes.chunks(2) {
            let temp = u16::from_be_bytes([pair[0], pair[1]]);
            sum += u32::from(temp);
        }

        while sum > 0xFFFF {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        if sum as u16 != 0xFFFF {
            return Err(IPv4Error::InvalidChecksum);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ipv4::{IPv4, IPv4Error, Protocol};

    #[test]
    fn ipv4_정상_파싱() {
        #[rustfmt::skip]
        let bytes = [
            0x45,           // Version=4, IHL=5 -> Header 20B
            0b101010_11,    // DSCP=42, ECN=3,
            0x00, 0x14,     // Total Length=20
            0x12, 0x34,     // Identification
            0x00, 0x00,     // Flags=0 Fragment Offset=0
            0x40,           // TTL=64
            0x01,           // Protocol=ICMP
            0x97, 0x51,     // Header Checksum
            192, 168, 0, 1, // Source IP
            8, 8, 8, 8      // Destination IP
        ];

        let ipv4 = IPv4::parse(&bytes).expect("정상 파싱 되어야함");

        assert_eq!(ipv4.version, 4);
        assert_eq!(ipv4.ihl, 5);
        assert_eq!(ipv4.dscp, 42);
        assert_eq!(ipv4.ecn, 3);
        assert_eq!(ipv4.total_length, 20);
        assert_eq!(ipv4.identification, 0x1234);
        assert_eq!(ipv4.flags, 0);
        assert_eq!(ipv4.fragment_offset, 0);
        assert_eq!(ipv4.ttl, 64);
        assert_eq!(ipv4.protocol, Protocol::ICMP);
        assert_eq!(ipv4.checksum, 0x9751);
        assert_eq!(ipv4.src_ip, [192, 168, 0, 1]);
        assert_eq!(ipv4.dst_ip, [8, 8, 8, 8]);
        assert_eq!(ipv4.options, vec![]);
    }

    #[test]
    fn ipv4_헤더짧음() {
        #[rustfmt::skip]
        let bytes = [
            0x45,           // Version=4, IHL=5 -> Header 20B
            0b101010_11,    // DSCP=42, ECN=3,
            0x00, 0x14,     // Total Length=20
            0x12, 0x34,     // Identification
            0x00, 0x00,     // Flags=0 Fragment Offset=0
            0x40,           // TTL=64
            0x01,           // Protocol=ICMP
            0xBE, 0xEF,     // Header Checksum
            192, 168, 0, 1, // Source IP
            // 8, 8, 8, 8   // Destination IP
        ];

        let ipv4 = IPv4::parse(&bytes);

        assert_eq!(ipv4, Err(IPv4Error::InvalidHeader));
    }

    #[test]
    fn ipv4_ihl_불일치() {
        #[rustfmt::skip]
        let bytes = [
            0x46,           // Version=4, IHL=6 -> Header 24B
            0b101010_11,    // DSCP=42, ECN=3,
            0x00, 0x14,     // Total Length=20
            0x12, 0x34,     // Identification
            0x00, 0x00,     // Flags=0 Fragment Offset=0
            0x40,           // TTL=64
            0x01,           // Protocol=ICMP
            0xBE, 0xEF,     // Header Checksum
            192, 168, 0, 1, // Source IP
            8, 8, 8, 8      // Destination IP
        ];

        let ipv4 = IPv4::parse(&bytes);

        assert_eq!(ipv4, Err(IPv4Error::InvalidByteLength));
    }

    #[test]
    fn ipv4_total_length_불일치() {
        #[rustfmt::skip]
        let bytes = [
            0x45,           // Version=4, IHL=5 -> Header 20B
            0b101010_11,    // DSCP=42, ECN=3,
            0x00, 0x10,     // Total Length=16
            0x12, 0x34,     // Identification
            0x00, 0x00,     // Flags=0 Fragment Offset=0
            0x40,           // TTL=64
            0x01,           // Protocol=ICMP
            0x97, 0x55,     // Header Checksum
            192, 168, 0, 1, // Source IP
            8, 8, 8, 8      // Destination IP
        ];

        let ipv4 = IPv4::parse(&bytes);

        assert_eq!(ipv4, Err(IPv4Error::InvalidHeader));
    }

    #[test]
    fn ipv4_options_포함() {
        #[rustfmt::skip]
        let bytes = [
            0x46,           // Version=4, IHL=6 -> Header 24B
            0b101010_11,    // DSCP=42, ECN=3,
            0x00, 0x18,     // Total Length=24
            0x12, 0x34,     // Identification
            0x00, 0x00,     // Flags=0 Fragment Offset=0
            0x40,           // TTL=64
            0x01,           // Protocol=ICMP
            0x3E, 0xB2,     // Header Checksum
            192, 168, 0, 1, // Source IP
            8, 8, 8, 8,     // Destination IP
            0x12, 0x34, 0x45, 0x67 // options
        ];

        let ipv4 = IPv4::parse(&bytes).expect("정상 파싱 되어야함");

        assert_eq!(ipv4.options, vec![0x12, 0x34, 0x45, 0x67]);
    }
}
