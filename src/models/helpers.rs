use mongodb::{bson};

#[macro_export]
macro_rules! bson_to_struct {
  ( $st:item | $( $x:expr => $t:ty ),* ) => {
    
  };
}

pub trait Inject {
  fn to_bson(&self) -> bson::ordered::OrderedDocument;
  fn create(&self, conn: &mongodb::Client) -> mongodb::coll::results::InsertOneResult;
}

