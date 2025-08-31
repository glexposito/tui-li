use crate::helpers::{seed_url, setup_service_with_client};
use actix_web::{
    http::{StatusCode, header},
    test,
};
mod helpers;

#[actix_web::test]
async fn test_redirect_url_not_found() {
    // Arrange
    let (service_data, _guard) = helpers::setup_service().await;
    let app = helpers::init_app(service_data.clone()).await;

    // Act
    let req = test::TestRequest::get().uri("/does-not-exist").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_redirect_url_found() {
    // Arrange
    let long_url = "https://example.com";
    let id = "g6teal";

    let (service_data, client, _guard) = setup_service_with_client().await;
    let app = helpers::init_app(service_data.clone()).await;

    seed_url(
        &client,
        "url",
        id,
        long_url,
        "2025-08-30T09:54:27.319522346+00:00",
        "100680ad546ce6a577f42f52df33b4cfdca756859e664b8d7de329b150d09ce9",
    )
    .await
    .unwrap();

    // Act
    let redirect_req = test::TestRequest::get()
        .uri(&format!("/{}", id))
        .to_request();
    let redirect_resp = test::call_service(&app, redirect_req).await;

    // Assert
    assert_eq!(redirect_resp.status(), StatusCode::FOUND);
    let location = redirect_resp
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, long_url);
}
