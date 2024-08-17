use lambda_http::{Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;

async fn lambda_handler(request: Request) -> Result<Response<Body>, Error> {
    let parsed = parse_request(request);
    let result = handler(parsed).await?;
    let resp = build_response(result);
    Ok(resp)
}

fn parse_request(request: Request) -> (String, u32) {
    let mut user = "cumet04".to_string();
    let mut per_page = 5;

    if let Some(query) = request.uri().query() {
        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "user" => user = value.to_string(),
                "per_page" => per_page = value.parse().unwrap(),
                _ => {}
            }
        }
    }
    (user, per_page)
}

fn build_response(result: Vec<String>) -> Response<Body> {
    Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!({"result": result}).to_string().into())
        .unwrap()
}

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
}

async fn handler((user, per_page): (String, u32)) -> Result<Vec<String>, Error> {
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
    async fn test_lambda_handler_no_input() {
        let event = Request::default();
        let result = lambda_handler(event).await;
        assert!(result.is_ok(), "error: {:?}", result.err());
        println!("test_no_input.result = {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_request_no_input() {
        let event: Request = http::Request::builder()
            .method("GET")
            .uri("http://example.com/")
            .body(Body::Empty)
            .unwrap();
        let (user, per_page) = parse_request(event);
        assert_eq!(user, "cumet04".to_string());
        assert_eq!(per_page, 5);
    }

    #[tokio::test]
    async fn test_parse_request_with_valid_params() {
        let event: Request = http::Request::builder()
            .method("GET")
            .uri("http://example.com/?user=rust-lang&per_page=3")
            .body(Body::Empty)
            .unwrap();
        let (user, per_page) = parse_request(event);
        assert_eq!(user, "rust-lang".to_string());
        assert_eq!(per_page, 3);
    }

    #[tokio::test]
    async fn test_handler_result_has_per_page_items() {
        let result1 = handler(("cumet04".to_string(), 5)).await.unwrap();
        assert_eq!(result1.len(), 5);

        let result2 = handler(("cumet04".to_string(), 7)).await.unwrap();
        assert_eq!(result2.len(), 7);

        let result3 = handler(("cumet04".to_string(), 3)).await.unwrap();
        assert_eq!(result3.len(), 3);
    }
}
