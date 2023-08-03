use std::collections::HashMap;

use uuid::Uuid;

use super::{customer::Customer, product::Product};

pub struct Purchase<'a> {
    pub customer: &'a Customer<'a>,
    pub products: &'a HashMap<Uuid, Product>,
    pub id: Uuid,
}
