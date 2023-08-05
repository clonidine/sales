use std::collections::HashMap;

use sqlx::types::Uuid;

use super::product::Product;

#[derive(Debug)]
pub struct Cart {
    pub id: Uuid,
    pub products: HashMap<Uuid, Product>,
}
