use lambda_http::{Body, Error, Request, Response};
use serde_json::json;

async fn lambda_handler(event: Request) -> Result<Response<Body>, Error> {
    // payloadやらcontextをparseしたりvalidationしたりする
    println!("{:?}", event);
    let result = handler().await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!({"result": result}).to_string().into())
        .map_err(Box::new)?;

    Ok(resp)
}

async fn handler() -> Result<String, Error> {
    let body = reqwest::get("https://www.example.com")
        .await?
        .text()
        .await?;
    println!("{body}");
    println!("=====");

    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("div > p > a").unwrap();
    let elements = document.select(&selector);

    let result = elements
        .map(|e| e.text().next().unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    return Ok(result);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::tracing::init_default_subscriber();
    lambda_http::run(lambda_http::service_fn(lambda_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_no_input() {
        let event = Request::default();
        let result = lambda_handler(event).await;
        assert!(result.is_ok(), "error: {:?}", result.err());
    }
}
