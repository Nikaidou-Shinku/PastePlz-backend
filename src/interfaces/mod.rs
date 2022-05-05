use serde::Serialize;

pub mod config;
pub mod paste;

#[derive(Serialize)]
pub struct Resp<T> {
  code: u16,
  msg: String,
  data: Option<T>
}

impl<T> Resp<T> {
  pub fn ok(val: T) -> Resp<T> {
    Resp::<T> {
      code: 0,
      msg: String::from(""),
      data: Some(val)
    }
  }

  pub fn err(err_code: u16, message: String) -> Resp<T> {
    Resp::<T> {
      code: err_code,
      msg: message,
      data: None
    }
  }
}
