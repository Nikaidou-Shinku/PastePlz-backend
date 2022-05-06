use actix_web::{App, HttpServer, web};

mod interfaces;
mod database;
mod routers;

use database::init_database;
use routers::{home, new::new_paste, get::get_paste};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    let pool = web::Data::new(init_database());
    App::new()
      .app_data(pool.clone())
      .service(home)
      .service(new_paste)
      .service(get_paste)
  })
  .bind(("127.0.0.1", 8521))?
  .run()
  .await
}
