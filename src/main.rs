mod entities;
mod routers;

use std::{net::SocketAddr, sync::Arc};

use axum::{
  routing::{get, post},
  Router,
};
use sea_orm::{Database, DatabaseConnection};

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub struct AppState {
  db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed!");

  let db = Database::connect("sqlite:./data.db?mode=rwc")
    .await
    .expect("Error opening database!");

  let app = Router::new()
    .route("/", get(|| async { "Hello, PastePlz!" }))
    .route("/paste", post(routers::new_paste))
    .route("/paste/:token", get(routers::get_paste))
    .with_state(Arc::new(AppState { db }));

  let addr = SocketAddr::from(([127, 0, 0, 1], 8521));

  tracing::info!("Listening on {addr}");

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .expect("Error while serving!");
}
