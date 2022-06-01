extern crate serde_json;

use futures::{stream, StreamExt};
use reqwest::Client;
use tokio;


#[tokio::main]
async fn main() {
  let mut conc_requests_size:usize = 1;
  println!("########  Adding item to table");
  create_item("2", "rice", conc_requests_size).await;
  println!("########");

  println!("########  Adding item to table");
  create_item("2", "fish", conc_requests_size).await;
  println!("########");

  println!("########  Adding item to table");
  create_item("2", "egg", conc_requests_size).await;
  println!("########");

  println!("########  Removing item from table");
  delete_items(2, "rice", conc_requests_size).await;
  println!("########");

  println!("########  Query remaining item for table");
  fetch_items(2, "", conc_requests_size).await;
  println!("########");

  println!("########  Show specified item for a specified table");
  fetch_items(2, "fish", conc_requests_size).await;
  println!("########");
  
  println!("########  CONCURRENT REQUESTS #######");
  
  conc_requests_size = 20;
  println!("########");

  println!("########  Adding item to table");
  create_item("2", "rice", conc_requests_size).await;
  println!("########");

  println!("########  Show specified item for a specified table");
  fetch_items(2, "fish", conc_requests_size).await;
  println!("########");

  println!("########  Removing item from table");
  delete_items(2, "rice", conc_requests_size).await;
  println!("########");
}

async fn create_item(table_id:&str, item_name: &str, conc_requests_size:usize) {
  let client = reqwest::Client::new();

  let url = vec!["http://127.0.0.1:8000/item"; conc_requests_size];

  let bodies = stream::iter(url)
    .map(|url| {
        let client = &client;
        async move {
            let resp = client.post(url).json(&serde_json::json!({
              "table_id": table_id.to_string(),
              "item_name": item_name.to_string()
            })).send().await?;
            resp.text().await
        }
    })
    .buffer_unordered(conc_requests_size);

  bodies.for_each(|b| async {
    match b {
        Ok(b) => println!("{:}", b),
        Err(e) => eprintln!("Got an error: {}", e),
    }
  }).await;
}

async fn fetch_items(table_id:u64, item_name: &str, conc_requests_size:usize) {
  let client = Client::new();

    let query_params = (table_id, item_name);
    let url_str = match query_params {
      (table_id, item_name) if (table_id != 0 && item_name != "") => format!("http://127.0.0.1:8000/item?table_id={}&item_name={}", table_id, item_name),
      (table_id, item_name) if (table_id != 0 && item_name == "") => format!("http://127.0.0.1:8000/item?table_id={}", table_id),
      (table_id, item_name) if (table_id == 0 && item_name != "") => format!("http://127.0.0.1:8000/item?item_name={}", item_name),
      (table_id, item_name) if (table_id == 0 && item_name == "") => format!("http://127.0.0.1:8000/item"),
      (_, _) => format!("http://127.0.0.1:8000/item"),
    };

    let url = vec![url_str; conc_requests_size];

    let bodies = stream::iter(url)
      .map(|url| {
          let client = &client;
          async move {
              let resp = client.get(url).send().await?;
              resp.text().await
          }
      })
      .buffer_unordered(conc_requests_size);

    bodies.for_each(|b| async {
      match b {
          Ok(b) => println!("{:}", b),
          Err(e) => eprintln!("Got an error: {}", e),
      }
    }).await;
}

// http://127.0.0.1:8000/item?table_id=5&item_name=deletable
async fn delete_items(table_id:u64, item_name: &str, conc_requests_size:usize) {
  let client = Client::new();
  let url_str = format!("http://127.0.0.1:8000/item?table_id={}&item_name={}", table_id, item_name);

    let url = vec![url_str; conc_requests_size];

    let bodies = stream::iter(url)
      .map(|url| {
          let client = &client;
          async move {
              let resp = client.delete(url).send().await?;
              resp.text().await
          }
      })
      .buffer_unordered(conc_requests_size);

    bodies.for_each(|b| async {
      match b {
          Ok(b) => println!("{:}", b),
          Err(e) => eprintln!("Got an error: {}", e),
      }
    }).await;
}
