use std::io::Result;

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9000").await?;

    let mut buf = [0; 1024];
    loop {
        if let Ok((amt, addr)) = socket.recv_from(&mut buf).await {
            let msg = String::from_utf8_lossy(&buf[..amt]);
            println!("{addr} recv: {msg}");
            
            let parts: Vec<&str> = msg.split(':').collect();

            if !matches!(parts[0], "DATA") {
                socket.send_to("ERR:INVALID_MSG".as_bytes(), addr).await?;
            }

            socket
                .send_to(format!("ACK:{}", parts[1]).as_bytes(), addr)
                .await?;
        }
    }
}
