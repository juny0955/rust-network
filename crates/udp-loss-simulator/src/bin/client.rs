use std::{
    io::Result,
    net::{SocketAddr, UdpSocket},
};

fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    for i in 0..10 {
        socket.send_to(format!("{}:hello", { i + 1 }).as_bytes(), server_addr)?;
    }
    socket.send_to("END".as_bytes(), server_addr)?;

    Ok(())
}
