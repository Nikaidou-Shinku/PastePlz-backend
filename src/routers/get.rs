use serde::Serialize;
use chrono::{DateTime, Local};
use deadpool_postgres::Pool;
use actix_web::{HttpResponse, Responder, web};

use crate::interfaces::ErrResp;

#[derive(Serialize)]
struct Res {
  lang: String,
  content: String,
  time: DateTime<Local>
}

#[actix_web::get("/paste/{token}")]
async fn get_paste((token, data): (
  web::Path<String>,
  web::Data<Pool>
)) -> impl Responder {
  let token = token.into_inner();
  let client = data.get().await.unwrap();
  let stmt = client.prepare_cached("SELECT lang, content, tm FROM paste WHERE token = $1;").await.unwrap();
  let rows = client.query(&stmt, &[&token]).await.unwrap();

  if rows.len() == 0 {
    return HttpResponse::NotFound()
      .json(ErrResp::<Res> {
        error: format!("Unable to find paste with token `{}`!", token),
        detail: None
      });
  }

  HttpResponse::Ok().json(Res {
    lang: rows[0].get("lang"),
    content: rows[0].get("content"),
    time: rows[0].get("tm")
  })
}
