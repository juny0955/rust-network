use std::{io::Result, net::SocketAddr, str::from_utf8};

use tokio::net::UdpSocket;

const DISCOVER_REQUEST: &[u8] = b"CHAT_SERVER";
const DISCOVER_RESPONSE: &[u8] = b"DISCOVER";

#[tokio::main]
async fn main() -> Result<()> {
    let broadcast_socket = UdpSocket::bind("0.0.0.0:9001").await?;
    broadcast_socket.set_broadcast(true)?;

    let socket = UdpSocket::bind("0.0.0.0:9000").await?;
    let mut clients: Vec<SocketAddr> = Vec::new();
    println!("Server is running");

    let mut bc_buf = [0; 1024];
    let mut buf = [0; 1024];
    loop {
        tokio::select! {
            discover = broadcast_socket.recv_from(&mut bc_buf) => {
                let (amt, socket_addr) = discover?;

                if &bc_buf[..amt] != DISCOVER_REQUEST {
                    println!("다른 메세지 무시: {:?}", &bc_buf[..amt]);
                    continue;
                }

                if !clients.contains(&socket_addr) {
                    println!("join client {socket_addr}");
                    clients.push(socket_addr);
                }

                broadcast_socket.send_to(DISCOVER_RESPONSE, socket_addr).await?;
            },
            recv = socket.recv_from(&mut buf) => {
                let (amt, socket_addr) = recv?;
                let msg = match from_utf8(&buf[..amt]) {
                    Ok(m) => format!("{socket_addr}: {m}"),
                    Err(e) => {
                        eprintln!("메세지 변환 오류: {e}");
                        continue;
                    },
                };

                for client in &clients {
                    if socket_addr != *client {
                        socket.send_to(msg.as_bytes(), client).await?;
                    }
                }
            }
        }
    }
}
