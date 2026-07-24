pub fn to_be_u16(value: u16) -> [u8; 2] {
    [(value >> 8) as u8, (value & 0x00FF) as u8]
}

pub fn from_be_u16(value: [u8; 2]) -> u16 {
    (value[0] as u16) << 8 | value[1] as u16
}

#[cfg(test)]
mod tests {
    use crate::endian::{from_be_u16, to_be_u16};

    #[test]
    fn u16_be_변환() {
        let bytes = 0x1234;
        let be = to_be_u16(bytes);

        assert_eq!(be[0], 0x12);
        assert_eq!(be[1], 0x34);
    }

    #[test]
    fn be_u16_변환() {
        let bytes = from_be_u16([0x12, 0x34]);
        assert_eq!(bytes, 0x1234);
    }
}
