pub mod db_impl;
pub mod mysql;

#[cfg(test)]
mod db_tests {
    use sales_api::api::provider::Database;

    use crate::db::db_impl::DBImplementation;

    #[test]
    fn opening_conn() {
        let db = DBImplementation::new();

        let conn = db.connect();

        assert!(conn.is_ok())
    }
}
