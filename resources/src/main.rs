pub mod date;
pub mod pages;
pub mod product;

use rocket::{routes, Error, Ignite, Rocket, Route};

#[rocket::main]
async fn main() -> Result<(), Error> {
    let routes = get_routes();

    let _rocket = launch(routes).await?;

    Ok(())
}

async fn launch(routes: Vec<Route>) -> Result<Rocket<Ignite>, rocket::Error> {
    Ok(rocket::build().mount("/", routes).launch().await?)
}

fn get_routes() -> Vec<Route> {
    routes![
        pages::index,
        date::controller::current_date,
        product::controller::find_one,
        product::controller::find_all,
        product::controller::update_stock,
        product::controller::delete
    ]
}
