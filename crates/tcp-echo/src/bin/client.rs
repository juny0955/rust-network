use std::{
    io::{ErrorKind, Result},
    net::SocketAddr,
    str::from_utf8,
    time::Duration,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));
    let mut stream = match TcpStream::connect(&addr).await {
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

    stream.write_all(b"hel").await?;
    stream.write_all(b"lo\n").await?;
    stream.write_all(b"w").await?;
    stream.write_all(b"orld\n").await?;

    let mut pending = Vec::new();
    let mut recv_frams = 0;

    while recv_frams < 2 {
        let mut buf = [0; 1024];
        let amt = stream.read(&mut buf).await?;

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
