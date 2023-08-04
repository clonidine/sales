use async_trait::async_trait;
use sqlx::{AnyConnection, Error};

#[async_trait]
pub trait Database {
    async fn connect() -> Result<AnyConnection, Error>;
    async fn get_db_url() -> Result<String, String>;
}
