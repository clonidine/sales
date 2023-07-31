use rust_decimal::Decimal;
use sales_api::api::repository::Repository;

static TABLE_NAME: &str = "products";

pub struct Product {
    name: String,
    id: usize,
    price: Decimal,
}

impl Product {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_price(&self) -> &Decimal {
        &self.price
    }
}

impl Repository for Product {
    fn save(&self) -> Result<(), String> {
        todo!()
    }

    fn delete(&self) -> Result<(), String> {
        todo!()
    }

    fn find_one(&self, id: usize) -> Option<&Self> {
        todo!()
    }

    fn create_table(&self) -> Result<(), String> {
        todo!()
    }

    fn get_table_name() -> &'static str {
        TABLE_NAME
    }
}
