use std::io;

use crate::cidr::Cidr;

mod cidr;
mod error;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("입력 읽기 중 오류");

    match Cidr::parse(input.trim()) {
        Ok(cidr) => {
            println!("cidr: {:?}", cidr);
            println!("subnet_mask: {:?}", cidr.subnet_mask());
            println!("network_address: {:?}", cidr.network_address());
        }
        Err(e) => eprintln!("{:?}", e),
    }
}
