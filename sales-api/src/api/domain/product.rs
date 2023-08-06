use sqlx::{types::BigDecimal, Type};

#[derive(
    sqlx::FromRow,
    sqlx::Encode,
    sqlx::Decode,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Type,
)]
pub struct Product {
    pub name: String,
    pub id: Option<u64>,
    pub price: BigDecimal,
}

impl Product {
    pub fn new(name: String, id: Option<u64>, price: BigDecimal) -> Product {
        Product { name, id, price }
    }
}
