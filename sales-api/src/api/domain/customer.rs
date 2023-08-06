use super::purchase::Purchase;

#[derive(Debug)]
pub struct Customer<'a> {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub purchases: Vec<Purchase<'a>>,
    pub id: i64,
}

impl<'a> Customer<'a> {
    pub fn new(name: String, email: String, phone: Option<String>, id: i64) -> Customer<'a> {
        let purchases = Vec::new();

        Customer {
            name,
            email,
            phone,
            purchases,
            id,
        }
    }
}
