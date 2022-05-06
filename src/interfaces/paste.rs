use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize)]
pub enum Language { // TODO: convert config to enum by using macro
  Markdown,
  Cpp,
  Python,
  Java,
  JavaScript,
  TypeScript,
  HTML,
  CSS
}

// impl Language {
//   pub fn from_str(qwq: &str) -> Language {
//     match qwq {
//       "cpp" => Language::Cpp,
//       "python" => Language::Python,
//       "java" => Language::Java,
//       "javascript" => Language::JavaScript,
//       "typescript" => Language::TypeScript,
//       "html" => Language::HTML,
//       "css" => Language::CSS,
//       _ => Language::Markdown
//     }
//   }

//   pub fn as_str(&self) -> &'static str {
//     match self {
//       Language::Markdown => "markdown",
//       Language::Cpp => "cpp",
//       Language::Python => "python",
//       Language::Java => "java",
//       Language::JavaScript => "javascript",
//       Language::TypeScript => "typescript",
//       Language::HTML => "html",
//       Language::CSS => "css"
//     }
//   }
// }

#[derive(Serialize)]
pub struct Paste {
  pub lang: Language,
  pub content: String,
  pub time: DateTime<Local>
}
