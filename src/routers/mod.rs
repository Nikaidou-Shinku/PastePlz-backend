use::actix_web::{HttpResponse, Responder};

pub mod new;

#[actix_web::get("/")]
async fn home() -> impl Responder {
  HttpResponse::Ok().body("Hello, PastePlz!")
}
