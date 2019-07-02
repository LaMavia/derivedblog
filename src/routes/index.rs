use std::path::PathBuf;
use rocket::response::{Redirect};
use rocket_contrib::{templates::Template};
use crate::models::db::MyDatabase;

// type RawStr = &'static str;
#[get("/<_foo..>", rank = 10)]
pub fn catch_all(_conn: MyDatabase, _foo: PathBuf) -> Template {
  let mut context = std::collections::HashMap::new();
  context.insert("title", "my title");
  context.insert("data", "Here's all the data you'll ever need!");

  Template::render("general", &context)
}

#[get("/")]
pub fn index() -> Redirect {
  Redirect::to("/home")
}