use sqlx::{
    types::{BigDecimal, Uuid},
    FromRow,
};

#[derive(FromRow, sqlx::Encode, Debug, PartialEq)]
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
