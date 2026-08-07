use std::{io::Result, net::SocketAddr, time::Duration};

use tokio::{net::UdpSocket, time::timeout};

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    let seq = 1;
    socket
        .send_to(format!("DATA:{seq}:hello").as_bytes(), server_addr)
        .await?;

    let mut buf = [0; 1024];
    for _ in 0..3 {
        match timeout(Duration::from_secs(1), socket.recv_from(&mut buf)).await {
            Ok(Ok((amt, _))) => {
                let msg = String::from_utf8_lossy(&buf[..amt]);
                let parts: Vec<&str> = msg.split(':').collect();

                let resp_seq = match parts[1].parse::<u64>() {
                    Ok(s) => s,
                    Err(_) => {
                        eprintln!("잘못되 응답");
                        break;
                    }
                };

                if matches!(parts[0], "ACK") && resp_seq == seq {
                    println!("메세지 전송 성공");
                } else {
                    eprintln!("메세지 전송 오류: {}", parts[1]);
                }

                break;
            }
            Ok(Err(e)) => return Err(e),
            Err(_) => println!("timeout 재전송 시도"),
        }
    }

    Ok(())
}
