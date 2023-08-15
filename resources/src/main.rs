pub mod controllers;
pub mod pages;

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
        controllers::product::find_one,
        controllers::product::find_all,
        controllers::date::current_date,
        controllers::product::save,
        controllers::product::delete
    ]
}
