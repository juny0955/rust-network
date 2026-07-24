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
            println!("input: {}/{}", formatting(cidr.ip), cidr.prefix_len);
            println!("subnet_mask: {}", formatting(cidr.subnet_mask()));
            println!("network_address: {}", formatting(cidr.network_address()));
            println!(
                "broadcast_address: {}",
                formatting(cidr.broadcast_address())
            );
        }
        Err(e) => eprintln!("{:?}", e),
    }
}

fn formatting(arr: [u8; 4]) -> String {
    format!("{}.{}.{}.{}", arr[0], arr[1], arr[2], arr[3])
}
