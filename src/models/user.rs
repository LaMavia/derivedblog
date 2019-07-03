use super::helpers::Inject;
use bcrypt;
use mongodb::bson;
use mongodb::{db::ThreadedDatabase, ThreadedClient};

#[derive(Debug)]
pub struct Model {
  pub _id: String,
  pub login: String,
  pub password: String,
  pub email: String,
  pub nickname: String,
}

impl From<mongodb::Document> for Model {
  fn from(doc: mongodb::Document) -> Model {
    Model {
      _id: doc.get_str("_id").unwrap().to_string(),
      login: doc.get_str("login").unwrap().to_string(),
      password: doc.get_str("password").unwrap().to_string(),
      email: doc.get_str("email").unwrap().to_string(),
      nickname: doc.get_str("nickname").unwrap().to_string(),
    }
  }
}

#[allow(dead_code)]
impl Model {
  pub fn by_one(conn: &mongodb::Client, filter: bson::ordered::OrderedDocument) -> Option<Model> {
    let col = conn.db("derivedblog").collection("users");
    match col.find_one(Some(filter), None) {
      Ok(cursor) => match cursor {
        Some(doc) => Some(Model::from(doc)),
        None => None,
      },
      Err(_) => None,
    }
  }

  pub fn by(
    conn: &mongodb::Client,
    filter: Option<bson::ordered::OrderedDocument>,
  ) -> Option<Vec<Model>> {
    let col = conn.db("derivedblog").collection("users");
    match col.find(filter, None) {
      Ok(cursor) => {
        let mut users: Vec<Model> = Vec::new();

        for r in cursor {
          if let Ok(user) = r {
            users.push(Model::from(user));
          }
        }

        Some(users)
      }
      Err(_) => None,
    }
  }

  pub fn by_id(id: String, conn: &mongodb::Client) -> Option<Model> {
    let mut _id: bson::oid::ObjectId;
    match bson::oid::ObjectId::with_string(&*id) {
      Ok(id) => _id = id,
      Err(_) => return None,
    };

    let mut filter = bson::ordered::OrderedDocument::new();
    filter.insert("_id", id);

    Model::by_one(conn, filter)
  }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Injectable {
  pub login: String,
  pub password: String,
  pub email: String,
  pub nickname: String,
}

impl Injectable {
  pub fn new<'a>(login: String, password: String, email: String, nickname: String) -> Injectable {
    let hash = bcrypt::hash(password, 4).unwrap();

    Injectable {
      login,
      password: hash,
      email,
      nickname,
    }
  }
}

impl Inject for Injectable {
  fn to_bson(&self) -> bson::ordered::OrderedDocument {
    doc! {
      "login": self.login.to_owned(),
      "password": self.password.to_owned(),
      "email": self.email.to_owned(),
      "nickname": self.nickname.to_owned(),
    }
  }

  fn create(&self, conn: &mongodb::Client) -> mongodb::coll::results::InsertOneResult {
    let col = conn.db("derivedblog").collection("users");
    col
      .insert_one(self.to_bson().clone(), None)
      .ok()
      .expect("Failed to insert a new user")
  }
}
