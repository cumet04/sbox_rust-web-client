use lambda_runtime::{service_fn, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(service_fn(lambda_handler)).await?;
    Ok(())
}

async fn lambda_handler(event: LambdaEvent<Value>) -> Result<Value, lambda_runtime::Error> {
    // payloadやらcontextをparseしたりvalidationしたりする
    println!("{:?}", event);
    let result = handler().unwrap();
    Ok(json!({ "result": result }))
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
