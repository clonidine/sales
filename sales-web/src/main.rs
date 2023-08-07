use rocket::{Ignite, Rocket, Route};

mod pages;
mod requests;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
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
        requests::product::find_all,
        requests::date::current_date,
        requests::product::find_one
    ]
}
