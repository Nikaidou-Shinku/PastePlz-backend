use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use rand::{seq::SliceRandom, thread_rng};
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{entities::paste, AppState};

#[derive(Deserialize)]
pub struct PastePayload {
  lang: String,
  content: String,
}

#[derive(Serialize)]
pub struct PasteResponse {
  token: String,
}

fn gen_token() -> String {
  // Base58
  const CHARSET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
  let mut rng = thread_rng();

  (0..8)
    .map(|_| CHARSET.choose(&mut rng))
    .flatten()
    .map(|&c| c as char)
    .collect()
}

#[tracing::instrument(name = "new", skip_all)]
pub async fn new_paste(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<PastePayload>,
) -> Result<Json<PasteResponse>, StatusCode> {
  if payload.content.len() > 102400 {
    tracing::warn!(len = payload.content.len(), "Content too long");
    return Err(StatusCode::BAD_REQUEST);
  }

  if payload.lang.len() > 8 {
    tracing::warn!(lang = payload.lang, "Language name too long");
    return Err(StatusCode::BAD_REQUEST);
  }

  let token = gen_token();
  let now = OffsetDateTime::now_local().map_err(|err| {
    tracing::error!(err = err.to_string(), "Error while getting current time");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  let new_paste = paste::ActiveModel {
    token: ActiveValue::set(token.clone()),
    lang: ActiveValue::set(payload.lang),
    content: ActiveValue::set(payload.content),
    time: ActiveValue::set(now),
  };

  new_paste.insert(&state.db).await.map_err(|err| {
    tracing::error!(err = err.to_string(), "Error while inserting paste");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  Ok(Json(PasteResponse { token }))
}
