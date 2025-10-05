use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

use chrono::Utc;
use rand::Rng;
use serde_json;

mod tick;
use crate::tick::TradeTick;


fn publish_ticks(mut stream: TcpStream, addr: SocketAddr) -> i64 {
    // Publish random prices as ticks
    let mut rng = rand::thread_rng();
    let mut i: i64 = 1;
    loop {
        let now = Utc::now().timestamp_nanos_opt().expect("Bad timestamp");
        let p: f64 = rng.gen_range(95.0..=105.0);
        let p = (p * 100.0).round() / 100.0;
        let s = rng.gen_range(1..=20);
        let trd = TradeTick {
            ts: now,
            price: p,
            size: s,
        };
        let mut msg = serde_json::to_string(&trd).expect("JSON serialization failed.");
        msg.push('\n');

        // Write to 127.0.0.1:9001
        println!(
            "Publishing `trade_id={}` from {} to 127.0.0.1:9001.",
            i, addr
        );
        if let Err(e) = stream.write_all(msg.as_bytes()) {
            eprintln!("Client disconnected: {}", e);
            break i;
        }
        sleep(Duration::from_millis(rng.gen_range(1..=50)));
        i += 1;
    }
}

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
