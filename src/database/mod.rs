use std::{fs::File, io::Read};
use tokio_postgres::NoTls;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime, Pool};

use crate::interfaces::config::Cfg;

pub fn init_database() -> Pool {
  let cfg_file = File::open("./config.toml");

  if let Ok(mut res) = cfg_file {
    let mut content = String::new();
    if let Err(_) = res.read_to_string(&mut content) {
      panic!("Unable to read config file(config.toml)!");
    } else {
      let global_cfg: Cfg = toml::from_str(&content).unwrap();
      let db_cfg = global_cfg.database;

      let mut cfg = Config::new();
      cfg.host = Some(db_cfg.hostname);
      cfg.port = Some(db_cfg.port);
      cfg.user = Some(db_cfg.username);
      cfg.password = Some(db_cfg.password);
      cfg.dbname = Some(db_cfg.dbname);
      cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
      cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
    }
  } else {
    panic!("Unable to open config file(config.toml)!");
  }
}
