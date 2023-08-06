use std::collections::HashMap;

use super::{customer::Customer, product::Product};

#[derive(Debug)]
pub struct Purchase<'a> {
    pub customer: &'a Customer<'a>,
    pub products: &'a HashMap<i64, Product>,
    pub id: i64,
}
