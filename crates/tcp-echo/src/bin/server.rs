use std::io::Result;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000").await?;
    println!("TCP server listening");

    loop {
        match listener.accept().await {
            Ok((mut stream, addr)) => {
                let mut pending = Vec::new();

                loop {
                    let mut buf = [0; 1024];
                    let amt = stream.read(&mut buf).await?;

                    if amt == 0 {
                        println!("{addr} 연결 종료");
                        break;
                    }

                    pending.extend_from_slice(&buf[..amt]);

                    while let Some(newline_idx) = pending.iter().position(|&byte| byte == b'\n') {
                        let frame: Vec<u8> = pending.drain(..=newline_idx).collect();
                        stream.write_all(&frame).await?;
                    }
                }
            }
            Err(e) => eprintln!("클라이언트 연결 오류: {e}"),
        }
    }
}
