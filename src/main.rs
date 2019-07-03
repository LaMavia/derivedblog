#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use(doc)] extern crate bson;
use rocket_contrib::{serve::StaticFiles, templates::Template};

extern crate serde;
extern crate mongodb;
extern crate dotenv;
extern crate bcrypt;

mod models;

mod routes;
use routes::index;

fn main() {
  match std::env::var("PORT") {
    Ok(_) => dotenv::dotenv().ok(),
    _ => dotenv::from_filename(".env.dev").ok(),
  };

  rocket::ignite()
    .attach(Template::fairing())
    .mount("/", routes![index::index, index::catch_all])
    .mount("/static", StaticFiles::from("./client/build/").rank(-6))
    .launch();
}
