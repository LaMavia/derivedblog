use std::path::PathBuf;
use rocket::response::{Redirect};
use rocket_contrib::{templates::Template};
use crate::models::{post, db};
// type RawStr = &'static str;
#[get("/<_foo..>", rank = 10)]
pub fn catch_all(_foo: PathBuf) -> Template {
  let mut context = std::collections::HashMap::new();
  context.insert("title", "my title");
  context.insert("data", "Here's all the data you'll ever need!");

  Template::render("general", &context)
}

#[get("/")]
pub fn index() -> Redirect {
  let p = post::Injectable::new(
    "Cats will rule the world!",
    "Get to know how",
    "Catty abstract followed by a little bit of lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque ac tincidunt ante. Sed fringilla nulla dolor, nec luctus neque volutpat et. Phasellus pellentesque lacus in massa blandit convallis. Nam interdum pharetra dolor, eget faucibus nibh congue pulvinar.",
    &vec!["cats".to_string(), "world".to_string(), "dev".to_string()],
    "6422a33e-94a4-4520-bc48-79a423afc877",
    "## Lorem ipsum dolor sit amet.
    consectetur adipiscing elit. Pellentesque ac tincidunt ante. Sed fringilla nulla dolor, nec luctus neque volutpat et. Phasellus pellentesque lacus in massa blandit convallis. Nam interdum pharetra dolor, eget faucibus nibh congue pulvinar. 
    ## Mauris fermentum ligula volutpat.
    vulputate turpis eget, viverra arcu. Sed a tristique magna. In vitae velit non augue condimentum pellentesque nec nec libero.",
    "#cccccc,#231f20,#b81434",
    "https://mocah.org/uploads/posts/4590305-nyan-cat-digital-art-space-art-cat-animals-colorful-humor-lsd-rainbows.jpg",
    "02-07-2019",
    "left"
  );

  let conn = db::connect();

  p.create(&conn);

  Redirect::to("/home")
}