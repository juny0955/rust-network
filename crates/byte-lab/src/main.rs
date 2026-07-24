use crate::endian::{from_be_u16, to_be_u16};

mod endian;

fn main() {
    let bytes = 0x1234;

    let to_be = to_be_u16(bytes);
    println!("to_be: {:?}", to_be);

    let from_be = from_be_u16(to_be);
    println!("from_be: {from_be}");
}
