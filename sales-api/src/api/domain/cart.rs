use std::collections::HashMap;

use sqlx::types::Uuid;

use super::product::Product;

#[derive(Debug)]
pub struct Cart {
    pub id: i64,
    pub products: HashMap<Uuid, Product>,
}
