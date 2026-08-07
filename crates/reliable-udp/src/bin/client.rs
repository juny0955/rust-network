use std::{io::Result, net::SocketAddr, time::Duration};

use tokio::{net::UdpSocket, time::timeout};

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    socket.send_to("DATA:1:hello".as_bytes(), server_addr).await?;

    let mut buf = [0; 1024];
    timeout(Duration::from_secs(1), async {
        if let Ok((amt, _)) = socket.recv_from(&mut buf).await {
            let msg = String::from_utf8_lossy(&buf[..amt]);
            let parts: Vec<&str> = msg.split(':').collect();

            if matches!(parts[0], "ACK") {
                println!("메세지 전송 성공");
            } else {
                eprintln!("메세지 전송 오류: {}", parts[1]);
            }
        }
    })
    .await?;

    Ok(())
}
