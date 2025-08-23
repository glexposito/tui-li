use actix_web::{
    App,
    http::{StatusCode, header},
    test, web,
};
use std::sync::Mutex;
use tui_li::{routes, services::shortener::UrlStore};

#[actix_web::test]
async fn test_redirect_url_not_found() {
    let store = web::Data::new(Mutex::new(UrlStore::new()));

    let app = test::init_service(
        App::new()
            .app_data(store.clone()) // ⬅️ inject the same state as main()
            .configure(routes::config),
    )
    .await;

    let req = test::TestRequest::get().uri("/does-not-exist").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_redirect_url_found() {
    let store = web::Data::new(Mutex::new(UrlStore::new()));
    let app =
        test::init_service(App::new().app_data(store.clone()).configure(routes::config)).await;

    // Create a short URL first
    let shorten_req = test::TestRequest::post()
        .uri("/shorten")
        .set_json(&serde_json::json!({ "long_url": "https://example.com" }))
        .to_request();
    let shorten_resp = test::call_service(&app, shorten_req).await;
    assert!(shorten_resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(shorten_resp).await;
    let id = body["id"].as_str().unwrap();

    // Now request the redirect
    let redirect_req = test::TestRequest::get()
        .uri(&format!("/{}", id))
        .to_request();
    let redirect_resp = test::call_service(&app, redirect_req).await;

    assert_eq!(redirect_resp.status(), StatusCode::FOUND);
    let location = redirect_resp
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, "https://example.com");
}
