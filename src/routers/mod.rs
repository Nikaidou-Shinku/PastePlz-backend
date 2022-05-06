use::actix_web::{HttpResponse, Responder};

pub mod new;
pub mod get;

#[actix_web::get("/")]
async fn home() -> impl Responder {
  HttpResponse::Ok().body("Hello, PastePlz!")
}
