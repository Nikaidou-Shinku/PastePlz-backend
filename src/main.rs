use actix_web::{App, HttpServer};

mod routers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(routers::home)
  })
  .bind(("127.0.0.1", 8521))?
  .run()
  .await
}
