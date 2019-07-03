use mongodb::{bson};
use mongodb::{db::ThreadedDatabase, ThreadedClient};

#[derive(Debug)]
pub struct Model {
  pub _id: String,
  pub title: String,
  pub subtitle: String,
  pub abstract_: String,
  pub tags: Vec<String>,
  pub author_id: String,
  pub body: String,
  pub theme: String, // Array<String> in the js
  pub image_url: String,
  pub date: String,
  pub front_page_style: String,
}

impl From<mongodb::Document> for Model {
  fn from(doc: mongodb::Document) -> Model {
    let mut tags: Vec<String> = vec![];

    for t in doc.get_array("tags").unwrap().into_iter() {
      tags.push(t.as_str().unwrap().to_string());
    }

    Model {
      _id: doc.get_str("_id").unwrap().to_string(),
      title: doc.get_str("title").unwrap().to_string(),
      subtitle: doc.get_str("subtitle").unwrap().to_string(),
      abstract_: doc.get_str("abstract_").unwrap().to_string(),
      tags,
      author_id: doc.get_str("author_id").unwrap().to_string(),
      body: doc.get_str("body").unwrap().to_string(),
      theme: doc.get_str("theme").unwrap().to_string(),
      image_url: doc.get_str("image_url").unwrap().to_string(),
      date: doc.get_str("date").unwrap().to_string(),
      front_page_style: doc.get_str("front_page_style").unwrap().to_string(),
    }
  }
}

#[allow(dead_code)]
impl Model {
  pub fn all(conn: &mongodb::Client) -> Vec<Model> {
    let mut res: Vec<Model> = vec![];

    let col = conn.db("derivedblog").collection("posts");
    let cursor = col.find(None, None).unwrap();

    for r in cursor {
      if let Ok(doc) = r {
        let post = Model::from(doc);

        res.push(post);
      }
    }

    res
  }
}


#[derive(Debug)]
pub struct Injectable {
  pub title: String,
  pub subtitle: String,
  pub abstract_: String,
  pub tags: Vec<String>,
  pub author_id: String,
  pub body: String,
  pub theme: String, // Array<String> in the js
  pub image_url: String,
  pub date: String,
  pub front_page_style: String,
}

impl Injectable {
  pub fn new<'a>(
    title: &'a str,
    subtitle: &'a str,
    abstract_: &'a str,
    tags: &'a Vec<String>,
    author_id: &'a str,
    body: &'a str,
    theme: &'a str,
    image_url: &'a str,
    date: &'a str,
    front_page_style: &'a str,
  ) -> Injectable {
    Injectable {
      title: title.to_owned(),
      subtitle: subtitle.to_owned(),
      abstract_: abstract_.to_owned(),
      tags: tags.to_owned(),
      author_id: author_id.to_owned(),
      body: body.to_owned(),
      theme: theme.to_owned(),
      image_url: image_url.to_owned(),
      date: date.to_owned(),
      front_page_style: front_page_style.to_owned(),
    }
  }
}

impl super::helpers::Inject for Injectable {
  fn to_bson(&self) -> bson::ordered::OrderedDocument {
    let mut tags = String::new();
    for t in self.tags.iter() {
      tags.push_str(&t);
    }
    doc! {
      "title": self.title.to_owned(),
      "subtitle": self.subtitle.to_owned(),
      "abstract_": self.abstract_.to_owned(),
      "tags": tags,
      "author_id": self.author_id.to_owned(),
      "body": self.body.to_owned(),
      "theme": self.theme.to_owned(),
      "image_url": self.image_url.to_owned(),
      "date": self.date.to_owned(),
      "front_page_style": self.front_page_style.to_owned(),
    }
  }

  fn create(&self, conn: &mongodb::Client) -> mongodb::coll::results::InsertOneResult {
    let col = conn.db("derivedblog").collection("posts");
    col
      .insert_one(self.to_bson().clone(), None)
      .ok()
      .expect("Failed to insert a new post")
  }
}