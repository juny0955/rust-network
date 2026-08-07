use std::{
    io::Result,
    net::{SocketAddr, UdpSocket},
};

fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    for i in 1..11 {
        if i % 5 == 0 {
            println!("dropped: {i}");
            continue;
        }

        socket.send_to(format!("{}:hello", { i }).as_bytes(), server_addr)?;
        println!("send: {i}");
    }
    socket.send_to("END".as_bytes(), server_addr)?;

    Ok(())
}
