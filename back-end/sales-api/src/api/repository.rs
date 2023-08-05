use async_trait::async_trait;
use sqlx::{Result, mysql::MySqlRow};

#[async_trait]
pub trait Repository<T> {
    async fn save(obj: &T) -> Result<MySqlRow>;
    async fn delete(id: usize) -> Result<()>;
    async fn find_one(id: usize) -> Result<T>;
    async fn create_table() -> Result<()>;
}
