use super::purchase::Purchase;

pub struct Customer<'a> {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub purchases: Vec<Purchase<'a>>,
}

impl<'a> Customer<'a> {
    pub fn new(name: String, email: String, phone: Option<String>) -> Customer<'a> {
        let purchases = Vec::new();

        Customer {
            name,
            email,
            phone,
            purchases,
        }
    }
}
