use std::sync::Arc;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  Json,
};
use sea_orm::EntityTrait;

use crate::{
  entities::{paste, prelude::*},
  AppState,
};

#[tracing::instrument(name = "get_paste", skip_all, fields(token = %token))]
pub async fn get_paste(
  State(state): State<Arc<AppState>>,
  Path(token): Path<String>,
) -> Result<Json<paste::Model>, StatusCode> {
  let paste = Paste::find_by_id(&token)
    .one(&state.db)
    .await
    .map_err(|err| {
      tracing::error!(err = err.to_string(), "Error while fetching paste");
      StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or_else(|| {
      tracing::warn!("Paste not found");
      StatusCode::NOT_FOUND
    })?;

  Ok(Json(paste))
}
