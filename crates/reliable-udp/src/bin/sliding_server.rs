use std::io::Result;

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9000").await?;

    let mut buf = [0; 1024];
    let mut last_recv_seq = 0;
    loop {
        if let Ok((amt, addr)) = socket.recv_from(&mut buf).await {
            let msg = String::from_utf8_lossy(&buf[..amt]);

            let parts: Vec<&str> = msg.split(':').collect();

            if !matches!(parts[0], "DATA") {
                socket.send_to("ERR:INVALID_MSG".as_bytes(), addr).await?;
            }

            let recv_seq = match parts[1].parse::<u64>() {
                Ok(s) => s,
                Err(_) => {
                    socket.send_to("ERR:INVALID_SEQ".as_bytes(), addr).await?;
                    continue;
                }
            };

            if recv_seq > last_recv_seq {
                last_recv_seq = recv_seq;
                println!("{addr} recv payload: {}", parts[2]);
            }

            socket
                .send_to(format!("ACK:{}", parts[1]).as_bytes(), addr)
                .await?;
        }
    }
}
