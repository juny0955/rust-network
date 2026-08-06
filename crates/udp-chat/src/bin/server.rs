use std::{io::Result, net::SocketAddr, str::from_utf8};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9000").await?;
    let mut clients: Vec<SocketAddr> = Vec::new();
    println!("Server is running");

    let mut buf = [0; 1024];
    loop {
        let (amt, socket_addr) = socket.recv_from(&mut buf).await?;
        let msg = match from_utf8(&buf[..amt]) {
            Ok(m) => format!("{socket_addr}: {m}"),
            Err(e) => {
                eprintln!("메세지 변환 오류: {e}");
                continue;
            },
        };

        if !clients.contains(&socket_addr) {
            println!("join client {socket_addr}");
            clients.push(socket_addr);
        }

        for client in &clients {
            if socket_addr != *client {
                socket.send_to(msg.as_bytes(), client).await?;
            }
        }
    }
}
