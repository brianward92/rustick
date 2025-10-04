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
        let p = rng.gen_range(95.0..=105.0);
        let p = (((p * 100.0) as i32) as f64) / 100.0;
        let s = rng.gen_range(1..=20);
        let trd = TradeTick {
            ts: now,
            price: p,
            size: s,
        };
        println!(
            "Trade {} at {} at price {} of size {}.",
            i, trd.ts, trd.price, trd.size
        );
    }
}
