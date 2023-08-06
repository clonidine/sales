use async_trait::async_trait;
use sqlx::{mysql::MySqlQueryResult, Result};

#[async_trait]
pub trait Repository<T> {
    async fn save(obj: &T) -> Result<MySqlQueryResult>;
    async fn delete(id: u64) -> Result<MySqlQueryResult>;
    async fn find_one(id: u64) -> Result<T>;
    async fn find_all() -> Result<Vec<T>>;
    async fn create_table() -> Result<MySqlQueryResult>;
}
