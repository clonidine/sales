use sqlx::types::BigDecimal;

#[derive(sqlx::FromRow)]
pub struct ProductDTO {
    pub name: String,
    pub price: BigDecimal,
    pub stock: Option<u64>,
    pub id: Option<u64>,
}

impl ProductDTO {
    pub fn new(name: &str, price: BigDecimal, stock: Option<u64>, id: Option<u64>) -> ProductDTO {
        ProductDTO {
            name: name.to_string(),
            price,
            stock,
            id,
        }
    }
}
