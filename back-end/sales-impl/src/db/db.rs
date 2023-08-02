use sales_api::api::provider::Database;

pub struct DatabaseImpl;

impl Database for DatabaseImpl {
    fn connect(&self) -> Result<sqlx::AnyConnection, sqlx::Error> {
        todo!()
    }

    fn get_db_url() -> Result<String, String> {
        todo!()
    }
}
