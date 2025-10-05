use std::net::TcpListener;

mod publish_ticks;
mod tick;
use crate::publish_ticks::publish_ticks;

fn main() -> std::io::Result<()> {
    // Reserve local address space
    let server_socket =
        TcpListener::bind("127.0.0.1:9001").expect("Could not bind to 127.0.0.1:9001");
    println!("Allocated on 127.0.0.1:9001");

    // Loop Publisher
    loop {
        // Wait for extenral process
        println!("Waiting for next client...");
        let (stream, addr) = server_socket.accept()?;

        // Someone connnected, time to publish
        println!("Publishing from {}", addr);
        let nticks: i64 = publish_ticks(stream, addr);
        println!("Published {} tick(s) from {}", nticks, addr);

        // They disconnected
    }
}
