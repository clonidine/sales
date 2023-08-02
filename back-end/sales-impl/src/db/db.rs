use sales_api::api::provider::Database;
use sqlx::Error;

pub struct DatabaseImpl;

impl Database for DatabaseImpl {
    fn connect() -> Result<sqlx::AnyConnection, Error> {
        todo!()
    }

    fn get_db_url() -> Result<String, String> {
        todo!()
    }
}
