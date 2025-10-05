use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

mod publish_ticks;
mod tick;
use crate::publish_ticks::publish_ticks;

fn wait_for_client(server_socket: &TcpListener) -> std::io::Result<(TcpStream, SocketAddr)> {
    println!("Waiting for next client...");
    let (stream, addr) = server_socket.accept()?;
    Ok((stream, addr))
}

fn main() -> std::io::Result<()> {
    // Reserve local address space
    let server_socket =
        TcpListener::bind("127.0.0.1:9001").expect("Could not bind to 127.0.0.1:9001");
    println!("Allocated on 127.0.0.1:9001");

    // Loop Publisher
    loop {
        let (stream, addr) = wait_for_client(&server_socket).expect("Failed to connect client!");
        println!("Publishing from {}", addr);
        let nticks: i64 = publish_ticks(stream, addr);
        println!("Published {} tick(s) from {}", nticks, addr);
        // They disconnected
    }
}
