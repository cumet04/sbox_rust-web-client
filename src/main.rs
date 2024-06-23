fn main() {
    let result = handler().unwrap();
    println!("{:?}", result);
}

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
