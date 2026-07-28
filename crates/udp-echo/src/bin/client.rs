use std::{
    io::{
        ErrorKind::{TimedOut, WouldBlock},
        Result,
    },
    net::UdpSocket,
    str::from_utf8,
    time::Duration,
};

const READ_TIMEOUT: Duration = Duration::from_secs(3);

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.set_read_timeout(Some(READ_TIMEOUT))?;

    let mut buf = [0; 1024];
    socket.send_to(b"hello", "127.0.0.1:7000")?;

    let (amt, src) = match socket.recv_from(&mut buf) {
        Ok((amt, src)) => (amt, src),
        Err(e) if matches!(e.kind(), TimedOut | WouldBlock) => {
            eprintln!("Timeout {}초 동안 응답없음", READ_TIMEOUT.as_secs());
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    match from_utf8(&buf[..amt]) {
        Ok(message) => println!("{src} -> recv: {}", message),
        Err(e) => eprintln!("Message 변환 실패: {e}"),
    }

    Ok(())
}
