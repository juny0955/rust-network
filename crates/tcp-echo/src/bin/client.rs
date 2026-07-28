use std::{
    io::{Read, Result, Write},
    net::{SocketAddr, TcpStream},
    str::from_utf8,
    time::Duration,
};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));
    let mut stream = TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT)?;
    stream.write_all(b"hello")?;

    let mut buf = [0; 1024];
    let amt = stream.read(&mut buf)?;

    match from_utf8(&buf[..amt]) {
        Ok(message) => println!("Message 수신: {}", message),
        Err(e) => eprintln!("Message 변환 실패: {e}"),
    }

    Ok(())
}
