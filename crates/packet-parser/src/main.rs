use crate::{ethernet::Ethernet, ipv4::IPv4};

mod ethernet;
mod ipv4;

fn main() {
    #[rustfmt::skip]
    let bytes = [
        // Ethernet Header: 14 Bytes
        0xaa, 0xbb, 0xcc, 0xdd, 
        0xee, 0xff, 0x11, 0x22, 
        0x33, 0x44, 0x55, 0x66,
        0x08, 0x00,

        // IPv4 Header: 20 Bytes
        0x45,           // version=4, IHL=5
        0b101010_11,    // DSCP=42, ECN=3
        0x00, 0x14,     // total length=20
        0x12, 0x34,     // identification
        0x00, 0x00,     // flags, fragment offset
        0x40,           // TTL=64
        0x01,           // protocol=ICMP
        0x97, 0x51,     // IPv4 header checksum
        192, 168, 0, 1, // source IP
        8, 8, 8, 8,     // destination IP
    ];

    let ethernet = match Ethernet::parse(&bytes) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Ethernet 패킷 파싱 실패: {e:?}");
            return;
        }
    };
    println!("Ethernet Packet: {ethernet:?}");

    let ipv4 = match IPv4::parse(&ethernet.payload) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("IPv4 패킷 파싱 실패: {e:?}");
            return;
        }
    };
    println!("IPv4 Packet: {ipv4:?}");
}
