use sqlx::{AnyConnection, Error};

pub trait Database {
    fn connect(&self) -> Result<AnyConnection, Error>;
    fn get_db_url() -> Result<String, String>;
}
