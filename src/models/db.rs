use mongodb::{Client, ThreadedClient};
use std::env;

#[allow(dead_code)]
pub fn connect() -> Client {
  let host = env::var("DATABASE_HOST").unwrap().to_string();
  let port = env::var("DATABASE_PORT")
    .unwrap()
    .to_string()
    .parse::<u16>()
    .unwrap();

  Client::connect(&host, port).expect("Failed to initialize mongodb connection")
}
