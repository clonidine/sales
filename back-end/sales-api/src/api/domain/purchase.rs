use std::collections::HashMap;

use sqlx::types::Uuid;

use super::{customer::Customer, product::Product};

#[derive(Debug)]
pub struct Purchase<'a> {
    pub customer: &'a Customer<'a>,
    pub products: &'a HashMap<Uuid, Product>,
    pub id: Uuid,
}
