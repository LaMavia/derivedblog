#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use(bson, doc)] extern crate bson;
use rocket_contrib::{serve::StaticFiles, templates::Template};

extern crate dotenv;

mod models;
use models::db::MyDatabase;

mod routes;
use routes::index;

fn main() {
  match std::env::var("PORT") {
    Ok(_) => dotenv::dotenv().ok(),
    _ => dotenv::from_filename(".env.dev").ok(),
  };

  rocket::ignite()
    .attach(Template::fairing())
    .attach(MyDatabase::fairing())
    .mount("/", routes![index::index, index::catch_all])
    .mount("/static", StaticFiles::from("./client/build/").rank(-6))
    .launch();
}
