use std::{io::Result, net::UdpSocket};

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9000")?;

    let mut buf = [0; 1024];
    let mut last_recived = 0;
    loop {
        if let Ok((amt, _)) = socket.recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..amt]);

            if msg == "END" {
                break;
            }
            
            if let Some((seq, _)) = msg.split_once(':') {
                match seq.parse::<u64>() {
                    Ok(s) => {
                        if last_recived + 1 != s {
                            println!("패킷 유실 감지");
                        }

                        last_recived = s;
                    }
                    Err(_) => eprintln!("잘못된 sequence: {seq}"),
                }
            } else {
                eprintln!("잘못된 메세지 형식");
                continue;
            }

            println!("recived: {msg:?}");
        }
    }

    Ok(())
}
