use serde::Serialize;

#[derive(Serialize)]
pub struct TradeTick {
    pub ts: i64,
    pub price: f64,
    pub size: u32,
}
