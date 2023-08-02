use std::collections::HashMap;

use super::{customer::Customer, product::Product};

pub struct Purchase<'a> {
    pub customer: &'a Customer<'a>,
    pub products: &'a HashMap<usize, Product>,
    pub id: usize,
}
