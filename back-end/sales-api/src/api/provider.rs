use async_trait::async_trait;
use sqlx::{AnyConnection, Result};

#[async_trait]
pub trait Database {
    async fn connect(&self) -> Result<AnyConnection>;
    async fn get_db_url(&self) -> Result<String>;
}
