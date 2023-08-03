use rust_decimal::Decimal;
use uuid::Uuid;

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
