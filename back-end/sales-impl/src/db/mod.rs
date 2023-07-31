pub mod db_adapter;
pub mod mysql;

#[cfg(test)]
mod db_tests {
    use sales_api::api::provider::Database;

    use crate::db::db_adapter::DBAdapter;

    #[test]
    fn opening_conn() {
        let db = DBAdapter::new();

        let conn = db.connect();

        assert!(conn.is_ok())
    }
}
