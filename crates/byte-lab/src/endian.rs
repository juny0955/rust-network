pub fn to_be_u16(value: u16) -> [u8; 2] {
    [(value >> 8) as u8, (value & 0x00FF) as u8]
}

pub fn from_be_u16(value: [u8; 2]) -> u16 {
    (value[0] as u16) << 8 | value[1] as u16
}

pub fn to_be_u32(value: u32) -> [u8; 4] {
    [
        (value >> 24) as u8,
        (value >> 16) as u8,
        (value >> 8) as u8,
        (value & 0x00FF) as u8,
    ]
}

pub fn from_be_u32(value: [u8; 4]) -> u32 {
    (value[0] as u32) << 24 | (value[1] as u32) << 16 | (value[2] as u32) << 8 | value[3] as u32
}

#[cfg(test)]
mod tests {
    use crate::endian::{from_be_u16, from_be_u32, to_be_u16, to_be_u32};

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

    #[test]
    fn u32_be_변환() {
        let bytes = 0x12345678;
        let be = to_be_u32(bytes);

        assert_eq!(be[0], 0x12);
        assert_eq!(be[1], 0x34);
        assert_eq!(be[2], 0x56);
        assert_eq!(be[3], 0x78);
    }

    #[test]
    fn be_u32_변환() {
        let bytes = from_be_u32([0x12, 0x34, 0x56, 0x78]);
        assert_eq!(bytes, 0x12345678);
    }
}
