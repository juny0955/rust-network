use crate::ethernet::Ethernet;

mod ethernet;

fn main() {
    let bytes = [
        0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x08, 0x00,
    ];

    let ethernet = match Ethernet::parse(&bytes) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Ethernet 패킷 파싱 실패: {e:?}");
            return;
        }
    };

    println!("Ethernet Packet: {ethernet:?}");
}
