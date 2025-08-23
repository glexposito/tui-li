use std::sync::Mutex;
use actix_web::{App, http::StatusCode, test, web};
use tui_li::routes;
use tui_li::services::shortener::UrlStore;

#[actix_web::test]
async fn test_health() {
    let store = web::Data::new(Mutex::new(UrlStore::new()));

    let app = test::init_service(
        App::new()
            .app_data(store.clone()) // ⬅️ inject the same state as main()
            .configure(routes::config),
    )
        .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}