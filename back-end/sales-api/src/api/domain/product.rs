use sqlx::{types::BigDecimal, FromRow};
use uuid::Uuid;

#[derive(FromRow, sqlx::Encode)]
pub struct Product {
    pub name: String,
    pub id: Uuid,
    pub price: BigDecimal,
}

impl Product {
    pub fn new(name: String, id: Uuid, price: BigDecimal) -> Product {
        Product { name, id, price }
    }
}
