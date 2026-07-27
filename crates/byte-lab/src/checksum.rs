pub fn network_checksum(bytes: &[u8]) -> u16 {
    let mut sum: u32 = 0;

    for pair in bytes.chunks(2) {
        let high = pair[0] as u32;

        let low = if pair.len() == 2 { pair[1] as u32 } else { 0 };

        sum += (high << 8) | low;

        while sum > 0xFFFF {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
    }

    !(sum as u16)
}

#[cfg(test)]
mod tests {
    use crate::checksum::network_checksum;

    #[test]
    fn checksum_계산() {
        let bytes = [0x00, 0x01, 0xF2, 0x03, 0xF4, 0xF5, 0xF6, 0xF7];
        assert_eq!(network_checksum(&bytes), 0x220D);
    }
}
