use crate::{data::*, db, error::Error::*, DBPool, Result};
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Deserialize)]
pub struct SearchQuery {
    table_id: Option<i32>,
    item_name: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteQuery {
    table_id: Option<i32>,
    item_name: Option<String>,
}

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
    let db = db::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn list_items_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let items = db::fetch_items(&db_pool, query.table_id, query.item_name)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &items.into_iter().map(|t| ItemResponse::of(t)).collect(),
    ))
}

pub async fn delete_item_handler(query: DeleteQuery, db_pool: DBPool) -> Result<impl Reply> {
  db::delete_item(&db_pool, query.table_id, query.item_name)
    .await
    .map_err(|e| reject::custom(e))?;
  Ok(StatusCode::OK)
}

pub async fn create_item_handler(body: ItemRequest, db_pool: DBPool) -> Result<impl Reply> {
  Ok(json(&ItemResponse::of(
      db::create_item(&db_pool, body)
          .await
          .map_err(|e| reject::custom(e))?,
  )))
}
