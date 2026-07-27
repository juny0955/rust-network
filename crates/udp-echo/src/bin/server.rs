use std::{io::Result, net::UdpSocket};

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:7000")?;
    println!("UDP server Binded");

    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        socket.send_to(&buf[..amt], src)?;
    }
}
