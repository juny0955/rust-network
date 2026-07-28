use std::{
    io::{Read, Result, Write},
    net::TcpListener,
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    println!("TCP server listening");

    loop {
        match listener.accept() {
            Ok((mut stream, addr)) => {
                let mut buf = [0; 1024];
                let amt = stream.read(&mut buf)?;

                if amt == 0 {
                    println!("{addr} 연결 종료");
                    return Ok(());
                }

                stream.write_all(&buf[..amt])?;
            }
            Err(e) => eprintln!("클라이언트 연결 오류: {e}"),
        }
    }
}
