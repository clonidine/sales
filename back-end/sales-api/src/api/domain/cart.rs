use std::collections::HashMap;

use super::product::Product;

pub struct Cart {
    pub id: usize,
    pub products: HashMap<usize, Product>,
}
