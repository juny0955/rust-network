use std::{
    io::{ErrorKind, Read, Result, Write},
    net::{SocketAddr, TcpStream},
    str::from_utf8,
    time::Duration,
};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));
    let mut stream = match TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT) {
        Ok(s) => s,
        Err(e) => {
            match e.kind() {
                ErrorKind::TimedOut => eprintln!("연결 Timeout {}초", CONNECT_TIMEOUT.as_secs()),
                ErrorKind::ConnectionRefused => eprintln!("서버 연결 실패"),
                _ => eprintln!("연결 오류: {e}"),
            }

            return Err(e);
        }
    };

    for message in ["hello", "world"] {
        stream.write_all(message.as_bytes())?;

        let mut buf = [0; 1024];
        let amt = stream.read(&mut buf)?;

        match from_utf8(&buf[..amt]) {
            Ok(message) => println!("Message 수신: {}", message),
            Err(e) => eprintln!("Message 변환 실패: {e}"),
        }
    }

    Ok(())
}
