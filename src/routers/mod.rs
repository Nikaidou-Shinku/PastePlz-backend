use::actix_web::{HttpResponse, Responder};

#[actix_web::get("/")]
async fn home() -> impl Responder {
  HttpResponse::Ok().body("Hello, PastePlz!")
}
