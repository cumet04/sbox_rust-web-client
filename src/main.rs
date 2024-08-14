use lambda_http::{Body, Error, Request, Response};
use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
}

async fn handler() -> Result<Vec<String>, Error> {
    let user = "cumet04";
    let per_page = 5;
    let url = format!(
        "https://api.github.com/users/{}/repos?per_page={}",
        user, per_page
    );

    let body = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "cumet04/sbox_rust-web-client") // GitHub APIではなんらかのUAが必須 refs https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#user-agent
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?;
    if !body.status().is_success() {
        return Err(Error::from(format!(
            "Request failed with status code: {}, body:\n{}",
            body.status(),
            body.text().await.unwrap()
        )));
    }
    let json: Vec<Repo> = body.json().await?;

    let result = json.iter().map(|repo| repo.name.clone()).collect();

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
        print!("test_no_input.result = {:?}", result);
    }
}
