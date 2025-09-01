use actix_web::{App, http::StatusCode, test};
use tui_li::routes;

#[actix_web::test]
async fn test_health() {
    let app = test::init_service(App::new().configure(routes::config)).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
