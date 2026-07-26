use crate::endian::{from_be_u16, from_be_u32, to_be_u16, to_be_u32};

mod endian;

fn main() {
    let bytes_u16 = 0x1234;
    let bytes_u32 = 0x12345678;

    let to_be_u16 = to_be_u16(bytes_u16);
    println!("to_be_u16: {:?}", to_be_u16);

    let from_be_u16 = from_be_u16(to_be_u16);
    println!("from_be_u16: 0x{from_be_u16:04X}");

    let to_be_u32 = to_be_u32(bytes_u32);
    println!("to_be_u16: {:?}", to_be_u32);

    let from_be_u32 = from_be_u32(to_be_u32);
    println!("from_be_u16: 0x{from_be_u32:08X}");
}
