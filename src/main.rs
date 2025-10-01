use chrono::Utc;

struct TradeTick {
    ts: i64,
    price: f64,
    size: u32,
}

fn main() {
    let now = Utc::now().timestamp_nanos_opt().expect("Bad timestamp");
    let trd = TradeTick {
        ts: now,
        price: 123.45,
        size: 5,
    };
    println!("Trade at {} at price {} of size {}.", trd.ts, trd.price, trd.size);
}
