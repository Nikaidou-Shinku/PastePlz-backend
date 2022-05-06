use serde::Serialize;

pub mod config;
// pub mod paste;

#[derive(Serialize)]
pub struct ErrResp<T> {
  pub error: String,
  pub detail: Option<T>
}
