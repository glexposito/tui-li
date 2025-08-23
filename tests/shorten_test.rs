use actix_web::{App, http::StatusCode, test, web};
use std::sync::Mutex;
use tui_li::{routes, services::shortener::UrlStore};

#[actix_web::test]
async fn test_shorten_url() {
    // Arrange
    let store = web::Data::new(Mutex::new(UrlStore::new()));

    let app =
        test::init_service(App::new().app_data(store.clone()).configure(routes::config)).await;

    // Act
    let req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&serde_json::json!({ "long_url": "https://example.com" }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["long_url"], "https://example.com");
    assert!(body["id"].as_str().map(|s| !s.is_empty()).unwrap_or(false));
}
