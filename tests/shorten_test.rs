// tests/shorten_test.rs
use actix_web::{http::StatusCode, test};
use serde_json::{Value, json};
mod helpers;

#[actix_web::test]
async fn test_shorten_url_ok() {
    // Arrange
    let (shortener_service, _guard) = helpers::setup_service().await;
    let app = helpers::init_app(shortener_service.clone()).await;

    // Act
    let req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&json!({ "long_url": "https://example.com/" }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::CREATED);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["long_url"], "https://example.com"); // trailing slash kept
    assert!(body["id"].as_str().map(|s| !s.is_empty()).unwrap_or(false));
}

#[actix_web::test]
async fn test_shorten_url_rejects_non_http_scheme() {
    // NOTE: this expects your handler to guard schemes:
    // if url.scheme() not in {"http","https"} -> return 400 JSON.
    // Arrange
    let (shortener_service, _guard) = helpers::setup_service().await;
    let app = helpers::init_app(shortener_service.clone()).await;

    // Act
    let req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&json!({ "long_url": "ftp://example.com/file" }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "invalid_url");
    assert!(body["message"].as_str().unwrap().contains("http/https"));
}
