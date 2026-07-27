use std::{io::Result, net::UdpSocket, str::from_utf8};

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    let mut buf = [0; 1024];
    socket.send_to(b"hello", "127.0.0.1:7000")?;

    let (amt, src) = socket.recv_from(&mut buf)?;
    match from_utf8(&buf[..amt]) {
        Ok(message) => println!("{src} -> recv: {}", message),
        Err(e) => eprintln!("Message 변환 실패: {e}"),
    }

    Ok(())
}
