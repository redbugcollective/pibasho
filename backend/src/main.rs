use rocket_aquadoggo::aquadoggo_fairing::AquadoggoFairing;
use rocket_aquadoggo::container::AquadoggoContainer;

mod routes;
mod infra;
mod rocket_aquadoggo;

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> String {
    "OK".to_string()
}

#[launch]
#[rocket::main]
async fn rocket() -> _ {
    rocket::build()
        .manage(AquadoggoContainer::default())
        .attach(infra::cors::cors_fairing())
        .attach(AquadoggoFairing::default())
        .mount("/", routes![hello])
        .mount("/panda", routes::panda_node::routes())
        .mount("/apps", routes::apps::routes())
}
