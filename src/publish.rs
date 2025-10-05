use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::thread::sleep;
use std::time::Duration;

use chrono::Utc;
use rand::Rng;
use serde_json;

use crate::tick::TradeTick;

pub const SERVER_ADDR: &str = "127.0.0.1:9001";

pub fn publish_ticks(mut stream: TcpStream, addr: SocketAddr) -> i64 {
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

        println!(
            "Publishing `trade_id={}` from {} to {}.",
            i, addr, SERVER_ADDR
        );
        if let Err(e) = stream.write_all(msg.as_bytes()) {
            eprintln!("Client disconnected: {}", e);
            break i;
        }
        sleep(Duration::from_millis(rng.gen_range(1..=50)));
        i += 1;
    }
}
