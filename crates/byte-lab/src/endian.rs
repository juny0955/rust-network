pub fn to_big_endian_bytes(value: u16) -> [u8; 2] {
    [(value >> 8) as u8, (value & 0x00FF) as u8]
}

#[cfg(test)]
mod tests {
    use crate::endian::to_big_endian_bytes;

    #[test]
    fn bytes_big_endian_변환() {
        let bytes = 0x1234;
        let be = to_big_endian_bytes(bytes);

        assert_eq!(be[0], 0x12);
        assert_eq!(be[1], 0x34);
    }
}
