pub mod product;

use async_trait::async_trait;
use domain::product::Product;
use sqlx::Result;

#[async_trait]
pub trait ProductRepositoryAbstract {
    async fn find_one(id: u64) -> Result<Product, String>;
    async fn find_all() -> Result<Vec<Product>, String>;
    async fn save(product: &Product) -> Result<bool, String>;
    async fn delete(id: u64) -> Result<bool, String>;
    async fn create_table() -> Result<(), String>;
    async fn update(
        column_filter_name: &str,
        column_to_update: &str,
        filter_value: &str,
        updated_value: &str,
    ) -> Result<(), String>;
}
