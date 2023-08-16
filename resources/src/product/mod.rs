pub mod controller;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StockUpdate {
    stock: u64,
}
