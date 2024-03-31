mod db;

use reqwest::{Client, Error as RequestError};
use sqlx::{PgPool, Error as SqlxError};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Example usage of the db module
    let database_url = "postgres://user:password@localhost/my_database";
    let pool = db::connect_to_database(database_url).await?;

    db::query_database(&pool).await?;

    // Example usage for a Google Cloud Function
    let cloud_function_url = "https://your-region-your-project-id.cloudfunctions.net/your-function-name";
    let response = make_request(cloud_function_url, Some(serde_json::json!({"key": "value"}))).await?;
    println!("Response from Cloud Function: {}", response);

    // Example usage for another endpoint
    let other_endpoint_url = "https://example.com/api/endpoint";
    let response = make_request(other_endpoint_url, None).await?;
    println!("Response from other endpoint: {}", response);

    Ok(())
}

async fn make_request(url: &str, body: Option<Value>) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let request = match body {
        Some(json_body) => client.post(url).json(&json_body),
        None => client.get(url),
    };

    let response = request.send().await?;
    let response_text = response.text().await?;

    Ok(response_text)
}
