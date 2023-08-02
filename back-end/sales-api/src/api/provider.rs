use sqlx::{AnyConnection, Error};

pub trait Database {
    fn connect() -> Result<AnyConnection, Error>;
    fn get_db_url() -> Result<String, String>;
}
