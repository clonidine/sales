use async_trait::async_trait;
use sqlx::{mysql::MySqlQueryResult, types::Uuid, Result};

#[async_trait]
pub trait Repository<T> {
    async fn save(obj: &T) -> Result<MySqlQueryResult>;
    async fn delete(id: Uuid) -> Result<MySqlQueryResult>;
    async fn find_one(id: Uuid) -> Result<T>;
    async fn create_table() -> Result<MySqlQueryResult>;
}
