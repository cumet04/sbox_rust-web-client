fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.example.com")?.text()?;
    println!("{body}");
    println!("=====");

    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("div > p > a").unwrap();
    let elements = document.select(&selector);

    elements.for_each(|e| println!("{}", e.text().next().unwrap()));

    return Ok(());
}
