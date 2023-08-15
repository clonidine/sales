use async_trait::async_trait;
use db::{databases::db_mysql::DbMySql, Database};
use domain::product::Product;
use dto::product::ProductDTO;
use mapper::product::ProductMapper;
use sqlx::Result;

use crate::ProductRepositoryAbstract;

pub struct ProductRepositoryMySQL;

const TABLE_NAME: &str = "products";

#[async_trait]
impl ProductRepositoryAbstract for ProductRepositoryMySQL {
    async fn find_one(id: u64) -> Result<Product, String> {
        let mut conn = DbMySql::connect().await?;

        let query = format!("SELECT * FROM {} WHERE id = ?", TABLE_NAME);

        let dto = sqlx::query_as::<_, ProductDTO>(&query)
            .bind(id)
            .fetch_one(&mut conn)
            .await;

        match dto {
            Ok(product_dto) => {
                let product = ProductMapper::unwrap(&product_dto);

                match product {
                    Ok(product) => Ok(product),
                    Err(e) => Err(format!("{}", e)),
                }
            }

            Err(e) => Err(format!("{}", e)),
        }
    }

    async fn find_all() -> Result<Vec<Product>, String> {
        let mut conn = DbMySql::connect().await?;

        let query = format!("SELECT * FROM {}", TABLE_NAME);

        let products_dto: Result<Vec<ProductDTO>> = sqlx::query_as::<_, ProductDTO>(&query)
            .fetch_all(&mut conn)
            .await;

        match products_dto {
            Ok(products_dto) => {
                let mut products = Vec::new();

                for product_dto in products_dto {
                    let product = ProductMapper::unwrap(&product_dto);

                    match product {
                        Ok(product) => products.push(product),
                        Err(e) => panic!("{}", e),
                    }
                }

                Ok(products)
            }

            Err(e) => Err(format!("{}", e)),
        }
    }

    async fn save(product: &Product) -> Result<bool, String> {
        let mut conn = DbMySql::connect().await?;

        let dto = ProductMapper::wrap(product);

        match dto {
            Ok(dto) => {
                let query = format!("INSERT INTO {} VALUES (?, ?, ?)", TABLE_NAME);

                let query_result = sqlx::query(&query)
                    .bind(&dto.name)
                    .bind(&dto.id)
                    .bind(&dto.price)
                    .execute(&mut conn)
                    .await;

                match query_result {
                    Ok(query_result) => Ok(query_result.rows_affected() == 1),

                    Err(e) => Err(format!("{}", e)),
                }
            }

            Err(e) => Err(format!("{}", e)),
        }
    }

    async fn delete(id: u64) -> Result<bool, String> {
        let mut conn = DbMySql::connect().await?;

        let query = format!("DELETE FROM {} WHERE id = ?", TABLE_NAME);

        let query_result = sqlx::query(&query).bind(id).execute(&mut conn).await;

        match query_result {
            Ok(query_result) => {
                dbg!("{}", query_result.rows_affected());

                if query_result.rows_affected() > 0 {
                    Ok(query_result.rows_affected() == 1)
                } else {
                    Err(format!("Product not found"))
                }
            }

            Err(e) => Err(format!("{}", e)),
        }
    }

    async fn create_table() -> Result<(), String> {
        let mut conn = DbMySql::connect().await?;

        let query = format!("
        CREATE TABLE IF NOT EXISTS {} 
        (name VARCHAR(255) NOT NULL, id BIGINT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT, price DECIMAL NOT NULL)
        ", TABLE_NAME);

        let _ = sqlx::query(&query).execute(&mut conn).await;

        Ok(())
    }
}

#[cfg(test)]
mod repo_tests {
    use std::str::FromStr;

    use bigdecimal::BigDecimal;
    use domain::product::Product;

    use crate::{product::ProductRepositoryMySQL, ProductRepositoryAbstract};

    #[tokio::test]
    async fn saving() {
        let created_table = ProductRepositoryMySQL::create_table().await;

        match created_table {
            Ok(_) => {
                let product =
                    Product::new("Macarronada", BigDecimal::from_str("34").unwrap(), None);

                let product_saved = ProductRepositoryMySQL::save(&product).await;

                assert!(product_saved.is_ok())
            }

            Err(e) => panic!("{}", e),
        }
    }
}
