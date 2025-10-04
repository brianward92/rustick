use chrono::Utc;
use rand::Rng;

struct TradeTick {
    ts: i64,
    price: f64,
    size: u32,
}

fn main() {
    let mut rng = rand::thread_rng();
    for i in 1..=10 {
        let now = Utc::now().timestamp_nanos_opt().expect("Bad timestamp");
        let x = rng.gen_range(1..=20);
        let trd = TradeTick {
            ts: now,
            price: 123.45,
            size: x,
        };
        println!(
            "Trade {} at {} at price {} of size {}.",
            i, trd.ts, trd.price, trd.size
        );
    }
}
