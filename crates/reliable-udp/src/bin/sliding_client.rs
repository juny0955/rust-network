use std::{collections::HashSet, io::Result, net::SocketAddr};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    let mut pending: HashSet<u64> = HashSet::new();
    for i in 1..=3 {
        socket
            .send_to(format!("DATA:{i}:hello").as_bytes(), server_addr)
            .await?;

        pending.insert(i);
    }

    let mut buf = [0; 1024];
    loop {
        let (amt, _) = socket.recv_from(&mut buf).await?;

        let msg = String::from_utf8_lossy(&buf[..amt]);
        let parts: Vec<&str> = msg.split(':').collect();

        let resp_seq = match parts[1].parse::<u64>() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("잘못된 응답");
                break;
            }
        };

        pending.remove(&resp_seq);

        if pending.is_empty() {
            println!("완료");
            break;
        }
    }

    Ok(())
}
