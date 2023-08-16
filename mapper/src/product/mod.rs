use std::str::FromStr;

use bigdecimal::{BigDecimal, ParseBigDecimalError};
use domain::product::Product;
use dto::product::ProductDTO;
use sqlx_core::types::BigDecimal as SqlxBigDecimal;

pub struct ProductMapper;

impl ProductMapper {
    pub fn unwrap(product_dto: &ProductDTO) -> Result<Product, ParseBigDecimalError> {
        let price = BigDecimal::from_str(&product_dto.price.to_string());

        match price {
            Ok(price) => {
                let product = Product::new(
                    &product_dto.name,
                    price,
                    product_dto.stock,
                    product_dto.id,
                );

                Ok(product)
            }

            Err(e) => panic!("{}", e),
        }
    }

    pub fn wrap(product: &Product) -> Result<ProductDTO, ParseBigDecimalError> {
        let price = SqlxBigDecimal::from_str(&product.price.to_string());

        match price {
            Ok(price) => {
                let product_dto = ProductDTO::new(&product.name, price, product.stock, product.id);

                Ok(product_dto)
            }

            Err(e) => panic!("{}", e),
        }
    }
}
