use serde::{Deserialize, Serialize};
use rand::Rng;
use deadpool_postgres::Pool;
use actix_web::{HttpResponse, Responder, web};

use crate::interfaces::ErrResp;

fn get_token() -> String {
  const CHARSET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
  let mut rng = rand::thread_rng();
  (0..8).map(|_| { // TODO: convert config to token length
    let idx = rng.gen_range(0..CHARSET.len());
    CHARSET[idx] as char
  }).collect()
}

#[derive(Deserialize)]
struct PasteForm {
  lang: String,
  content: String
}

#[derive(Serialize)]
struct Res {
  token: String
}

#[actix_web::post("/paste")]
async fn new_paste((form, data): (
  web::Json<PasteForm>,
  web::Data<Pool>
)) -> impl Responder {
  if form.content.len() > 102400 { // TODO: convert config to maxlength
    return HttpResponse::BadRequest()
      .json(ErrResp::<Res> {
        error: String::from("Content exceeds maximum length!"),
        detail: None
      });
  }

  let token = get_token();

  let client = data.get().await.unwrap();
  let stmt = client.prepare_cached("INSERT INTO paste (token, lang, content, tm) VALUES ($1, $2, $3, now());").await.unwrap();
  client.query(&stmt, &[&token, &form.lang, &form.content]).await.unwrap();

  HttpResponse::Created().json(Res { token })
}
