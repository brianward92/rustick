use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn get_server_socket(addr: &str) -> std::io::Result<TcpListener> {
    let server_socket = TcpListener::bind(addr)?;
    println!("Allocated on {}", addr);
    Ok(server_socket)
}

pub fn wait_for_client(server_socket: &TcpListener) -> std::io::Result<(TcpStream, SocketAddr)> {
    println!("Waiting for next client...");
    let (stream, addr) = server_socket.accept()?;
    Ok((stream, addr))
}
