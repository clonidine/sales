use rust_decimal::Decimal;

pub struct Product {
    pub name: String,
    pub id: usize,
    pub price: Decimal,
}

impl Product {
    pub fn new(name: String, id: usize, price: Decimal) -> Product {
        Product { name, id, price }
    }
}
