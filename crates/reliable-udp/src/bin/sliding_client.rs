use std::{collections::HashSet, io::Result, net::SocketAddr};

use tokio::net::UdpSocket;

const WINDOW_SIZE: u64 = 3;
const TOTAL_DATA_SIZE: u64 = 10;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket
        .connect(SocketAddr::from(([127, 0, 0, 1], 9000)))
        .await?;

    let mut pending: HashSet<u64> = HashSet::new();
    let mut next_seq = 0;
    for i in 0..WINDOW_SIZE {
        send_window(i, &socket, &mut pending).await?;
        next_seq += 1;
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
        println!("resp seq: {resp_seq}");

        let rm = pending.remove(&resp_seq);
        if rm {
            if pending.is_empty() && next_seq >= TOTAL_DATA_SIZE {
                println!("완료");
                break;
            } else {
                if next_seq < TOTAL_DATA_SIZE {
                    send_window(next_seq, &socket, &mut pending).await?;
                    next_seq += 1;
                }
            }
        }
    }

    Ok(())
}

async fn send_window(seq: u64, socket: &UdpSocket, pending: &mut HashSet<u64>) -> Result<()> {
    socket.send(format!("DATA:{seq}:hello").as_bytes()).await?;
    pending.insert(seq);
    Ok(())
}
