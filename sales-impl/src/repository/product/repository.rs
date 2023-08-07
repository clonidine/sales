use async_trait::async_trait;
use sales_api::api::{
    domain::product::{Product, ProductWrapper},
    provider::Database,
    repository::Repository,
};
use sqlx::{mysql::MySqlQueryResult, Result};

use crate::db::db::DatabaseImpl;

pub struct ProductRepository;

const TABLE_NAME: &str = "products";

#[async_trait]
impl Repository<Product> for ProductRepository {
    async fn save(obj: &Product) -> Result<MySqlQueryResult> {
        let mut conn = DatabaseImpl::connect().await?;

        let query = format!("INSERT INTO {} VALUES (?, ?, ?)", TABLE_NAME);

        let obj = ProductWrapper::wrap(&obj);

        match obj {
            Ok(obj) => Ok(sqlx::query(&query)
                .bind(&obj.name)
                .bind(&obj.id)
                .bind(&obj.price)
                .execute(&mut conn)
                .await?),

            Err(e) => panic!("{}", e),
        }
    }

    async fn delete(id: u64) -> Result<MySqlQueryResult> {
        let mut conn = DatabaseImpl::connect().await?;

        let query = format!("DELETE FROM {} where id = ?", TABLE_NAME);

        let row = sqlx::query(&query).bind(id).execute(&mut conn).await?;

        Ok(row)
    }

    async fn find_one(id: u64) -> Result<Product> {
        let mut conn = DatabaseImpl::connect().await?;

        let query = format!("SELECT * FROM {} where id = ? LIMIT 1", TABLE_NAME);

        let wrapper = sqlx::query_as::<_, ProductWrapper>(&query)
            .bind(id)
            .fetch_one(&mut conn)
            .await?;

        let product = Product::parse(&wrapper);

        match product {
            Ok(product) => Ok(product),
            Err(e) => panic!("{}", e),
        }
    }

    async fn find_all() -> Result<Vec<Product>> {
        let mut conn = DatabaseImpl::connect().await?;

        let query = format!("SELECT * FROM {}", TABLE_NAME);

        let products: Vec<ProductWrapper> = sqlx::query_as::<_, ProductWrapper>(&query)
            .fetch_all(&mut conn)
            .await?;

        let products = products
            .iter()
            .map(|wrapper| Product::parse(wrapper))
            .collect::<Result<Vec<Product>, _>>()
            .ok()
            .unwrap_or_else(Vec::new);

        Ok(products)
    }

    async fn create_table() -> Result<MySqlQueryResult> {
        let mut conn = DatabaseImpl::connect().await?;

        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} (name VARCHAR(255) NOT NULL, id BIGINT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT, price DECIMAL NOT NULL)"
        , TABLE_NAME);

        let result = sqlx::query(&query).execute(&mut conn).await?;

        Ok(result)
    }
}

#[cfg(test)]
mod repository_tests {

    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use sales_api::api::{domain::product::Product, repository::Repository};

    use crate::repository::product::repository::ProductRepository;

    #[tokio::test]
    async fn inserting_product() {
        let created_table = ProductRepository::create_table().await;

        match created_table {
            Ok(_) => {
                let default_product = Product::new(
                    String::from("Burger"),
                    None,
                    Decimal::from_f64(36.5).unwrap(),
                );

                let saved_product = ProductRepository::save(&default_product).await;

                assert!(saved_product.is_ok())
            }

            Err(e) => panic!("Error while trying to create table: {}", e),
        }
    }
}
