use std::io;

use crate::cidr::Cidr;

mod cidr;
mod error;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("입력 읽기 중 오류");

    let cidr = Cidr::parse(input.trim());
}
