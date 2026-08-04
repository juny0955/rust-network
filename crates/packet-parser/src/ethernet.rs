#[derive(Debug, PartialEq, Eq)]
pub enum EtherType {
    IPv4,
    IPv6,
    ARP,
}

#[derive(Debug, PartialEq)]
pub struct Ethernet {
    pub des_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ether_type: EtherType,
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EthernetError {
    WrongBytes,
    WrongEtherType,
}

impl Ethernet {
    pub fn parse(bytes: &[u8]) -> Result<Self, EthernetError> {
        if bytes.len() < 14 {
            eprintln!("Ethernet Frame 바이트 길이가 짧습니다.");
            return Err(EthernetError::WrongBytes);
        }

        let des_mac = Self::parse_des_mac_addr(bytes);
        let src_mac: [u8; 6] = Self::parse_src_mac_addr(bytes);
        let ether_type = Self::parse_ether_type(bytes)?;
        let payload = bytes[14..].to_vec();

        Ok(Self {
            des_mac,
            src_mac,
            ether_type,
            payload,
        })
    }

    fn parse_des_mac_addr(bytes: &[u8]) -> [u8; 6] {
        let mut des_mac: [u8; 6] = [0, 0, 0, 0, 0, 0];
        des_mac.copy_from_slice(&bytes[..6]);
        des_mac
    }

    fn parse_src_mac_addr(bytes: &[u8]) -> [u8; 6] {
        let mut src_mac: [u8; 6] = [0, 0, 0, 0, 0, 0];
        src_mac.copy_from_slice(&bytes[6..12]);
        src_mac
    }

    fn parse_ether_type(bytes: &[u8]) -> Result<EtherType, EthernetError> {
        let type_byte = u16::from_be_bytes([bytes[12], bytes[13]]);

        Ok(match type_byte {
            0x0800 => EtherType::IPv4,
            0x0806 => EtherType::ARP,
            0x86DD => EtherType::IPv6,
            _ => {
                eprintln!("Ethernet Frame EtherType 바이트가 잘못되었습니다.");
                return Err(EthernetError::WrongEtherType);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ethernet::{EtherType, Ethernet, EthernetError};

    #[test]
    fn 정상_파싱() {
        let bytes = [
            0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x08, 0x00,
            0x12, 0x34,
        ];

        let ethernet = Ethernet::parse(&bytes).expect("정상 파싱되어야함");

        assert_eq!(ethernet.des_mac, [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
        assert_eq!(ethernet.src_mac, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        assert_eq!(ethernet.ether_type, EtherType::IPv4);
        assert_eq!(ethernet.payload, [0x12, 0x34]);
    }

    #[test]
    fn byte길이_짧음() {
        let bytes = [
            0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
        ];

        let ethernet = Ethernet::parse(&bytes);

        assert_eq!(ethernet, Err(EthernetError::WrongBytes));
    }

    #[test]
    fn ether_type_틀림() {
        let bytes = [
            0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x08, 0x90,
            0x12, 0x34,
        ];

        let ethernet = Ethernet::parse(&bytes);

        assert_eq!(ethernet, Err(EthernetError::WrongEtherType));
    }
}
