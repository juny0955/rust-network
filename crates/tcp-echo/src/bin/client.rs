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

    stream.write_all(b"hel")?;
    stream.write_all(b"lo\n")?;
    stream.write_all(b"w")?;
    stream.write_all(b"orld\n")?;

    let mut pending = Vec::new();
    let mut recv_frams = 0;

    while recv_frams < 2 {
        let mut buf = [0; 1024];
        let amt = stream.read(&mut buf)?;

        if amt == 0 {
            eprintln!("모든 Echo 응답 수신 전 서버 연결 종료됨");
            break;
        }

        pending.extend_from_slice(&buf[..amt]);

        while let Some(newline_idx) = pending.iter().position(|&byte| byte == b'\n') {
            let frames: Vec<u8> = pending.drain(..=newline_idx).collect();

            match from_utf8(&frames) {
                Ok(message) => println!("Message 수신: {message}"),
                Err(e) => eprintln!("Message 변환 실패: {e}"),
            };

            recv_frams += 1;
        }
    }

    Ok(())
}
