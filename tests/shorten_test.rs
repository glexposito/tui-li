// tests/shorten_test.rs
use actix_web::{App, http::StatusCode, test, web};
use serde_json::{Value, json};
use std::sync::Mutex;
use tui_li::{routes, services::shortener::UrlStore};

#[actix_web::test]
async fn test_shorten_url_ok() {
    // Arrange
    let store = web::Data::new(Mutex::new(UrlStore::new()));
    let app =
        test::init_service(App::new().app_data(store.clone()).configure(routes::config)).await;

    // Act
    let req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&json!({ "long_url": "https://example.com" }))
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
    let store = web::Data::new(Mutex::new(UrlStore::new()));
    let app =
        test::init_service(App::new().app_data(store.clone()).configure(routes::config)).await;

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

#[actix_web::test]
async fn test_shorten_url_malformed_url() {
    // Arrange: JSON error handler so deserialization failures return consistent JSON.
    use actix_web::{HttpResponse, error::InternalError};
    let json_cfg = web::JsonConfig::default().error_handler(|err, _req| {
        let body = json!({
            "error": "invalid_request",
            "message": "Malformed JSON or invalid URL."
        });
        InternalError::from_response(err, HttpResponse::BadRequest().json(body)).into()
    });

    let store = web::Data::new(Mutex::new(UrlStore::new()));
    let app = test::init_service(
        App::new()
            .app_data(json_cfg)
            .app_data(store.clone())
            .configure(routes::config),
    )
    .await;

    // Act
    let req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&json!({ "long_url": "not a url" }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "invalid_request");
    assert!(
        body["message"].as_str().unwrap().contains("invalid URL")
            || body["message"].as_str().unwrap().contains("Malformed")
    );
}
