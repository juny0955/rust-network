use std::{io::Result, net::SocketAddr, str::from_utf8, time::Duration};

use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    net::UdpSocket,
    time::timeout,
};

const DISCOVER_REQUEST: &[u8] = b"CHAT_SERVER";
const DISCOVER_RESPONSE: &[u8] = b"DISCOVER";
const DISCOVER_TIMEOUT: Duration = Duration::from_secs(3);

const DISCOVER_PORT: u16 = 9001;
const CHAT_SERVER_PORT: u16 = 9000;

async fn discover_chat_server(socket: &UdpSocket) -> Result<SocketAddr> {
    // let broadcast_addr = SocketAddr::from(([192, 168, 35, 82], DISCOVER_PORT));
    let broadcast_addr = SocketAddr::from(([127, 0, 0, 1], DISCOVER_PORT));
    let mut buf = [0; 1024];

    socket.send_to(DISCOVER_REQUEST, broadcast_addr).await?;

    loop {
        let (amt, server_addr) = timeout(DISCOVER_TIMEOUT, socket.recv_from(&mut buf)).await??;

        if &buf[..amt] != DISCOVER_RESPONSE {
            println!("다른 메세지 무시: {:?}", &buf[..amt]);
            continue;
        }

        println!("CHAT SERVER FOUND!");
        return Ok(SocketAddr::from((server_addr.ip(), CHAT_SERVER_PORT)));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;

    let server_addr = discover_chat_server(&socket).await?;
    socket.connect(server_addr).await?;
    socket.set_broadcast(false)?;

    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    let mut recv_buf = [0; 1024];
    loop {
        tokio::select! {
            recv = socket.recv(&mut recv_buf) => {
                match recv {
                    Ok(amt) => {
                        match from_utf8(&recv_buf[..amt]) {
                            Ok(m) => println!("{m}"),
                            Err(e) => eprintln!("메세지 변환 오류: {e:?}"),
                        }
                    },
                    Err(e) => eprintln!("메세지 수신 오류: {e}"),
                }
            },
            input = lines.next_line() => {
                match input {
                    Ok(Some(line)) => socket.send(line.as_bytes()).await?,
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
