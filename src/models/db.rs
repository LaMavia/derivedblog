use rocket_contrib::databases::mongodb;
 
#[database("derivedblog")]
pub struct MyDatabase(mongodb::db::Database);