use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Item {
    pub id: i32,
    pub table_id: i32,
    pub created_at: DateTime<Utc>,
    pub prep_time: i32,
    pub item_name: String,
}

#[derive(Deserialize, Debug)]
pub struct ItemRequest {
    pub table_id: String,
    pub item_name: String,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: i32,
    pub table_id: i32,
    pub prep_time: i32,
    pub item_name: String,
}

impl ItemResponse {
    pub fn of(item: Item) -> ItemResponse {
        ItemResponse {
            id: item.id,
            table_id: item.table_id,
            prep_time: item.prep_time,
            item_name: item.item_name,
        }
    }
}
