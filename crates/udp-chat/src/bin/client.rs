use std::{io::Result, net::SocketAddr, str::from_utf8};

use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    net::UdpSocket,
};

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    let mut recv_buf = [0; 1024];

    loop {
        tokio::select! {
            recv = socket.recv_from(&mut recv_buf) => {
                match recv {
                    Ok((amt, addr)) => {
                        match from_utf8(&recv_buf[..amt]) {
                            Ok(m) => println!("{m}"),
                            Err(e) => eprintln!("{addr} 메세지 변환 오류: {e:?}"),
                        }
                    },
                    Err(e) => eprintln!("메세지 수신 오류: {e}"),
                }
            },
            input = lines.next_line() => {
                match input {
                    Ok(Some(line)) => socket.send_to(line.as_bytes(), server_addr).await?,
                    Ok(None) => {
                        println!("Ctrl-D 종료");
                        break;
                    },
                    Err(e) => {
                        eprintln!("표준 입력 오류: {e:?}");
                        continue;
                    },
                };
            }
        }
    }

    Ok(())
}
