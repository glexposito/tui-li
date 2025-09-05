use actix_governor::Governor;
use actix_web::{App, http::StatusCode, test};
use std::net::SocketAddr;
use tui_li::middleware::rate_limiter::rate_limiter_config;
use tui_li::routes;

#[actix_web::test]
async fn test_rate_limiter() {
    // apply the global rate limiter (1 token/s, burst 20)
    let conf = rate_limiter_config();
    let app = test::init_service(
        App::new()
            .wrap(Governor::new(&conf))
            .configure(routes::config),
    )
    .await;

    // Use a fixed peer IP so requests share the same bucket.
    let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();

    let mut ok = 0usize;
    let mut too_many = 0usize;

    // A bit flaky...
    // Send a quick burst; expect at least one 429 once the burst is exhausted.
    for _ in 0..25 {
        let req = test::TestRequest::get()
            .uri("/health")
            .peer_addr(addr) // <-- critical for PeerIpKeyExtractor
            .to_request();

        let resp = test::call_service(&app, req).await;
        match resp.status() {
            s if s == StatusCode::OK => ok += 1,
            s if s == StatusCode::TOO_MANY_REQUESTS => too_many += 1,
            _ => {}
        }
    }

    assert!(
        ok >= 10,
        "expected many OKs (burst allows ~20); got ok={ok}, 429={too_many}"
    );
    assert!(
        too_many >= 1,
        "expected at least one 429 after bursting; got ok={ok}, 429={too_many}"
    );
}
