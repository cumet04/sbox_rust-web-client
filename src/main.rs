use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(lambda_handler)).await
}

async fn lambda_handler(event: Request) -> Result<Response<Body>, Error> {
    // payloadやらcontextをparseしたりvalidationしたりする
    println!("{:?}", event);
    let result = handler().unwrap();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!({"result": result}).to_string().into())
        .map_err(Box::new)?;

    Ok(resp)
}

// fn main() {
//     let result = handler().unwrap();
//     println!("{:?}", result);
// }

fn handler() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.example.com")?.text()?;
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
