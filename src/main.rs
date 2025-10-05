use std::net::TcpListener;

mod publish;
mod tick;

fn main() -> std::io::Result<()> {
    // Reserve local address space
    let server_socket =
        TcpListener::bind("127.0.0.1:9001").expect("Could not bind to 127.0.0.1:9001");
    println!("Allocated on 127.0.0.1:9001");

    // Loop Publisher
    loop {
        let (stream, addr) =
            publish::wait_for_client(&server_socket).expect("Failed to connect client!");
        println!("Publishing from {}", addr);
        let nticks: i64 = publish::publish_ticks(stream, addr);
        println!("Published {} tick(s) from {}", nticks, addr);
        // They disconnected
    }
}
