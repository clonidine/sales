use std::collections::HashMap;

use uuid::Uuid;

use super::product::Product;

pub struct Cart {
    pub id: Uuid,
    pub products: HashMap<Uuid, Product>,
}
