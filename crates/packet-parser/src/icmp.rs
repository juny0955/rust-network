#[derive(Debug, PartialEq, Eq)]
pub struct Icmp {
    pub icmp_type: IcmpType,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence_number: u16,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IcmpType {
    Reply,
    Request,
}

impl IcmpType {
    pub fn parse(byte: u8) -> Result<Self, IcmpError> {
        match byte {
            0 => Ok(IcmpType::Reply),
            8 => Ok(IcmpType::Request),
            _ => Err(IcmpError::UnsupportedType),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IcmpError {
    InvalidByteLength,
    InvalidChecksum,
    UnsupportedType,
}

impl Icmp {
    pub fn parse(bytes: &[u8]) -> Result<Self, IcmpError> {
        if bytes.len() < 8 {
            return Err(IcmpError::InvalidByteLength);
        }

        let icmp_type = IcmpType::parse(bytes[0])?;
        let code = bytes[1];

        Self::validate_checksum(&bytes)?;

        let checksum = u16::from_be_bytes([bytes[2], bytes[3]]);
        let identifier = u16::from_be_bytes([bytes[4], bytes[5]]);
        let sequence_number = u16::from_be_bytes([bytes[6], bytes[7]]);
        let data = bytes[8..].to_vec();

        Ok(Self {
            icmp_type,
            code,
            checksum,
            identifier,
            sequence_number,
            data,
        })
    }

    fn validate_checksum(bytes: &[u8]) -> Result<(), IcmpError> {
        let mut sum: u32 = 0;

        let mut pairs = bytes.chunks_exact(2);
        for pair in &mut pairs {
            let temp = u16::from_be_bytes([pair[0], pair[1]]);
            sum += u32::from(temp);
        }

        if let [last] = pairs.remainder() {
            sum += u32::from(*last) << 8;
        }

        while sum > 0xFFFF {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        if sum as u16 != 0xFFFF {
            return Err(IcmpError::InvalidChecksum);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::icmp::{Icmp, IcmpError, IcmpType};

    #[test]
    fn icmp_정상_파싱() {
        let bytes = [8, 0, 0xE5, 0xCA, 0x12, 0x34, 0x00, 0x01];

        let icmp = Icmp::parse(&bytes).expect("정상 파싱 되어야함");

        assert_eq!(icmp.icmp_type, IcmpType::Request);
        assert_eq!(icmp.code, 0);
        assert_eq!(icmp.checksum, 0xE5CA);
        assert_eq!(icmp.identifier, 0x1234);
        assert_eq!(icmp.sequence_number, 1);
    }

    #[test]
    fn icmp_최소_길이_오류() {
        let bytes = [8, 0, 0xE5, 0xCA, 0x12, 0x34, 0x00];

        let icmp = Icmp::parse(&bytes);

        assert_eq!(icmp, Err(IcmpError::InvalidByteLength));
    }

    #[test]
    fn icmp_checksum_오류() {
        let bytes = [8, 0, 0xE5, 0xC1, 0x12, 0x34, 0x00, 0x01];

        let icmp = Icmp::parse(&bytes);

        assert_eq!(icmp, Err(IcmpError::InvalidChecksum));
    }

    #[test]
    fn 홀수_bytes_checksum_통과() {
        let bytes = [8, 0, 0x3A, 0xCA, 0x12, 0x34, 0x00, 0x01, 0xAB];

        let icmp = Icmp::parse(&bytes).expect("정상 파싱 되어야함");

        assert_eq!(icmp.icmp_type, IcmpType::Request);
        assert_eq!(icmp.code, 0);
        assert_eq!(icmp.checksum, 0x3ACA);
        assert_eq!(icmp.identifier, 0x1234);
        assert_eq!(icmp.sequence_number, 1);
        assert_eq!(icmp.data, vec![0xAB]);
    }
}
