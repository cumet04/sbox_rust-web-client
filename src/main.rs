fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.example.com")?.text()?;

    println!("{body}");
    return Ok(());
}
