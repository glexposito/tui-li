use actix_web::{
    App,
    http::{StatusCode, header},
    test, web,
};
use std::sync::Mutex;
use tui_li::{routes, services::shortener::UrlStore};

#[actix_web::test]
async fn test_redirect_url_not_found() {
    // Arrange
    let store = web::Data::new(Mutex::new(UrlStore::new()));

    let app = test::init_service(
        App::new()
            .app_data(store.clone())
            .configure(routes::config),
    )
    .await;

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
    let mut url_store = UrlStore::new();
    let url_mapping = url_store.add_url(long_url.to_string());
    let store = web::Data::new(Mutex::new(url_store));

    let app =
        test::init_service(App::new().app_data(store.clone()).configure(routes::config)).await;

    // Act
    let redirect_req = test::TestRequest::get()
        .uri(&format!("/{}", url_mapping.id))
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
    assert_eq!(location, "https://example.com");
}
