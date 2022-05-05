use serde::Deserialize;

#[derive(Deserialize)]
pub struct Cfg {
  pub database: DatabaseCfg
}

#[derive(Deserialize)]
pub struct DatabaseCfg {
  pub hostname: String,
  pub port: u16,
  pub username: String,
  pub password: String,
  pub dbname: String
}
