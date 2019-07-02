use rocket_contrib::databases::mongodb::{bson, Bson, db::DatabaseInner};
use crate::models::db;

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

#[allow(dead_code)]
impl Model {
  pub fn new<'a>(
    id: &'a str,
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
  ) -> Model {
    Model {
      _id: id.to_owned(),
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

  pub fn to_bson(&self) -> bson::ordered::OrderedDocument {
    let mut tags = String::new();
    for t in self.tags.iter() {
      tags.push_str(&t);
    }
    doc! {
      "_id": self._id.to_owned(),
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

  pub fn create(&self, conn: &db::MyDatabase) -> Result<Option<bson::ordered::OrderedDocument>, std::io::Error> {
    let client = conn.client.as_ref().db("");
  }
}
