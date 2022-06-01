use crate::{data::*, error, error::Error::*, DBCon, DBPool};
use chrono::prelude::*;
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls, Row};

type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 100;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";
const TABLE: &str = "item";
const SELECT_FIELDS: &str = "id, table_id, created_at, prep_time, item_name";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
  let init_file = fs::read_to_string(INIT_SQL)?;
  let con = get_db_con(db_pool).await?;
  con.batch_execute(init_file.as_str())
    .await
    .map_err(DBInitError)?;
  Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
  db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
  let config = Config::from_str("postgres://postgres@127.0.0.1:5432/postgres")?;

  let manager = PgConnectionManager::new(config, NoTls);
  Ok(Pool::builder()
    .max_open(DB_POOL_MAX_OPEN)
    .max_idle(DB_POOL_MAX_IDLE)
    .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
    .build(manager))
}

pub async fn fetch_items(db_pool: &DBPool, table_id: Option<i32>, item_name: Option<String>) -> Result<Vec<Item>> {
  let con = get_db_con(db_pool).await?;

  let where_clause = match (&table_id, &item_name) {
    (Some(_table_id), Some(_item_name)) => "WHERE table_id=$1 AND item_name=$2",
    (Some(_table_id), _) => "WHERE table_id=$1",
    (_, Some(_item_name)) => "WHERE item_name=$1",
    _ => "",
  };

  let query = format!(
    "SELECT {} FROM {} {} ORDER BY created_at DESC",
    SELECT_FIELDS, TABLE, where_clause
  );

  let q = match (table_id, item_name) {
    (Some(table_id), Some(item_name)) => con.query(query.as_str(), &[&table_id, &item_name]).await,
    (Some(table_id), _) => con.query(query.as_str(), &[&table_id]).await,
    (_, Some(item_name)) => con.query(query.as_str(), &[&item_name]).await,
    _ => con.query(query.as_str(), &[]).await,
  };

  let rows = q.map_err(DBQueryError)?;

  Ok(rows.iter().map(|r| row_to_item(&r)).collect())
}

pub async fn delete_item(db_pool: &DBPool, table_id: Option<i32>, item_name: Option<String>) -> Result<u64> {
  let con = get_db_con(db_pool).await?;
  let query = format!("DELETE FROM {} WHERE table_id = $1 AND item_name = $2", TABLE);
  con.execute(query.as_str(), &[&table_id, &item_name])
      .await
      .map_err(DBQueryError)
}

pub async fn create_item(db_pool: &DBPool, body: ItemRequest) -> Result<Item> {

  let con = get_db_con(db_pool).await?;
  let query = format!("INSERT INTO {} (table_id, item_name) VALUES ($1, $2) RETURNING *", TABLE);
  let row = con
      .query_one(query.as_str(), &[&body.table_id.parse::<i32>().unwrap(), &body.item_name])
      .await
      .map_err(DBQueryError)?;
  Ok(row_to_item(&row))
}

fn row_to_item(row: &Row) -> Item {
    let id: i32 = row.get(0);
    let table_id: i32 = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let prep_time: i32 = row.get(3);
    let item_name: String = row.get(4);
    Item {
        id,
        table_id,
        created_at,
        prep_time,
        item_name,
    }
}
