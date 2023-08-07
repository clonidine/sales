use std::collections::HashMap;

use super::product::Product;

#[derive(Debug)]
pub struct Cart {
    pub id: i64,
    pub products: HashMap<u64, Product>,
}
