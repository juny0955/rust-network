use crate::endian::to_big_endian_bytes;

mod endian;

fn main() {
    println!("{:?}", to_big_endian_bytes(0x1234));
}
