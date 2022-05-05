use::actix_web::{HttpResponse, Responder};
use chrono::Local;

use crate::interfaces::{
  Resp,
  paste::{
    Paste,
    Language
  }
};

#[actix_web::post("/new")]
async fn new_paste() -> impl Responder {
  HttpResponse::Ok()
  .json(Resp::ok(Paste {
    lang: Language::cpp,
    content: String::from("qwq"),
    time: Local::now()
  }))
}
