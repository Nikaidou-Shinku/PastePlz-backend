use serde::Serialize;

pub mod config;

#[derive(Serialize)]
pub struct ErrResp<T> {
  pub error: String,
  pub detail: Option<T>
}
