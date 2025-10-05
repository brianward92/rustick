mod publish;
mod server;
mod tick;

fn main() -> std::io::Result<()> {
    // Reserve local address space
    let server_socket = server::get_server_socket(publish::SERVER_ADDR)?;

    // Loop Publisher
    loop {
        let (stream, addr) = server::wait_for_client(&server_socket)?;
        println!("Publishing from {}", addr);
        let nticks = publish::publish_ticks(stream, addr);
        println!("Published {} tick(s) from {}", nticks, addr);
        // They disconnected
    }
}
