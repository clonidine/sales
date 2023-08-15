use bigdecimal::BigDecimal as Big;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Product {
    pub name: String,
    pub price: Big,
    pub id: Option<u64>,
}

impl Product {
    pub fn new(name: &str, price: Big, id: Option<u64>) -> Self {
        Product {
            name: name.to_string(),
            price,
            id,
        }
    }
}
