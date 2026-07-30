use std::{io::Result, str::from_utf8, time::Duration};

use tokio::{net::UdpSocket, time::timeout};

const READ_TIMEOUT: Duration = Duration::from_secs(3);

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    // socket.set_read_timeout(Some(READ_TIMEOUT)).await?;

    let mut buf = [0; 1024];
    socket.send_to(b"hello", "127.0.0.1:7000").await?;

    let (amt, src) = match timeout(READ_TIMEOUT, socket.recv_from(&mut buf)).await {
        Ok(Ok((amt, src))) => (amt, src),
        Ok(Err(e)) => return Err(e),
        Err(_) => {
            eprintln!("Timeout {}초 동안 응답없음", READ_TIMEOUT.as_secs());
            return Ok(());
        }
    };

    match from_utf8(&buf[..amt]) {
        Ok(message) => println!("{src} -> recv: {}", message),
        Err(e) => eprintln!("Message 변환 실패: {e}"),
    }

    Ok(())
}
