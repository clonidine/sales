use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, serde::Deserialize, sqlx::Encode)]
pub struct Product {
    pub name: String,
    pub id: Uuid,
    pub price: Decimal,
}

impl Product {
    pub fn new(name: String, id: Uuid, price: Decimal) -> Product {
        Product { name, id, price }
    }
}
