use std::collections::HashMap;
use std::process;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client, Response};
use std::env;

fn client_factory() -> Client {
    let notion_api_token: String = match env::var("NOTION_API_TOKEN") {
        Ok(val) => format!("Bearer {}", val),
        Err(e) => {
            println!("{}: {}", e, "NOTION_API_KEY");
            process::exit(1)
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&notion_api_token).unwrap(),
    );
    headers.insert("Notion-Version", HeaderValue::from_static("2022-06-28"));
    Client::builder().default_headers(headers).build().unwrap()
}

async fn get_page(page_id: &str) -> Response {
    let client = client_factory();
    client
        .get(format!(
            "https://api.notion.com/v1/blocks/{}/children",
            page_id
        ))
        .send()
        .await
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = get_page("7e94045672674171ab6c8bafc4682fa8").await;
    let page = response
        .json::<HashMap<String, serde_json::Value>>()
        .await?;
    let log_ids: Vec<_> = page
        .get("results")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p.get("id").unwrap().as_str().unwrap())
        .collect();
    let mut logs: Vec<HashMap<String, serde_json::Value>> = Vec::new();
    println!("{}", log_ids.len());
    for id in log_ids.iter() {
        println!("{}", id);
        let log = get_page(id).await;
        logs.push(log.json::<HashMap<String, serde_json::Value>>().await?);
    }
    println!("{:?}", logs);
    Ok(())
}
