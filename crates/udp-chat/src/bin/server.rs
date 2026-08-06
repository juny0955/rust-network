use std::{io::Result, net::SocketAddr};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9000").await?;
    let mut clients: Vec<SocketAddr> = Vec::new();
    println!("Server is running");

    let mut buf = [0; 1024];
    loop {
        let (amt, socket_addr) = socket.recv_from(&mut buf).await?;
        let msg = &buf[..amt];

        if !clients.contains(&socket_addr) {
            println!("join client {socket_addr}");
            clients.push(socket_addr);
        }

        for client in &clients {
            socket.send_to(msg, client).await?;
        }
    }
}
