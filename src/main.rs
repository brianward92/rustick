use std::io::Write;
use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;

use chrono::Utc;
use rand::Rng;

struct TradeTick {
    ts: i64,
    price: f64,
    size: u32,
}

fn main() -> std::io::Result<()> {
    // Reserve local address space
    let server_socket =
        TcpListener::bind("127.0.0.1:9001").expect("Could not bind to 127.0.0.1:9001");
    println!("Allocated on 127.0.0.1:9001");

    // Connect from this process
    let (mut stream, addr) = server_socket.accept()?;
    println!("Publishing from {}", addr);

    // Publish random prices as ticks
    let mut rng = rand::thread_rng();
    let mut i = 1;
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
        let msg = format!(
            "trade_id={} at {} at price {:.2} of size {}.",
            i, trd.ts, trd.price, trd.size
        );

        // Write to 127.0.0.1:9001
        stream.write_all(msg.as_bytes())?;
        println!(
            "Published `trade_id={}` from {} to 127.0.0.1:9001.",
            i, addr
        );
        sleep(Duration::from_millis(rng.gen_range(1..=50)));
        i += 1;
    }
}
