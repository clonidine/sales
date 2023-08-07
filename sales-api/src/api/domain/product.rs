use std::str::FromStr;

use rust_decimal::Decimal;
use sqlx::Type;
use sqlx_core::types::BigDecimal;

#[derive(
    sqlx::FromRow, sqlx::Encode, Debug, PartialEq, Type, serde::Serialize, serde::Deserialize,
)]
pub struct Product {
    pub name: String,
    pub id: Option<u64>,
    pub price: Decimal,
}

#[derive(sqlx::Encode, sqlx::Decode, sqlx::FromRow)]
pub struct ProductWrapper {
    pub name: String,
    pub id: Option<u64>,
    pub price: BigDecimal,
}

impl ProductWrapper {
    pub fn new(name: String, id: Option<u64>, price: BigDecimal) -> ProductWrapper {
        ProductWrapper { name, id, price }
    }

    pub fn wrap(product: &Product) -> Result<Self, String> {
        let price = BigDecimal::from_str(product.price.to_string().as_str());

        match price {
            Ok(price) => {
                let wrapper = ProductWrapper::new(product.name.clone(), product.id, price);

                Ok(wrapper)
            }

            Err(e) => panic!("Cannot wrap because of price: {}", e),
        }
    }
}

impl Product {
    pub fn new(name: String, id: Option<u64>, price: Decimal) -> Product {
        Product { name, id, price }
    }

    pub fn parse(wrapper: &ProductWrapper) -> Result<Self, String> {
        let price = Decimal::from_str(wrapper.price.to_string().as_str());

        match price {
            Ok(price) => {
                let model = Product::new(wrapper.name.clone(), wrapper.id, price);

                Ok(model)
            }
            Err(e) => panic!("Cannot parse price: {}", e),
        }
    }
}
