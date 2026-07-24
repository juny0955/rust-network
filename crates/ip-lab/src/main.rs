use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("입력 읽기 중 오류");

    let input = input.trim();

    let Some((ip, prefix_len)) = input.split_once("/") else {
        eprintln!("올바른 CIDR 형식이 아닙니다");
        return;
    };

    println!("ip = {ip}");
    println!("prefix = {prefix_len}");
}
