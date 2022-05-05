use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize)]
pub enum Language { // TODO: convert config to enum by using macro
  markdown,
  cpp,
  python,
  java,
  javascript,
  typescript,
  html,
  css
}

#[derive(Serialize)]
pub struct Paste {
  pub lang: Language,
  pub content: String,
  pub time: DateTime<Local>
}
