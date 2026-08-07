use std::{io::Result, net::UdpSocket};

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9000")?;

    let mut buf = [0; 1024];
    
    loop {
        if let Ok((amt, _)) = socket.recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..amt]);  

            if msg == "END" {
                break;
            }

            println!("{msg:?}");
        }
    }

    Ok(())
}
