use actix_governor::{
    GovernorConfig,
    GovernorConfigBuilder,
    PeerIpKeyExtractor,                   // re-exported at crate root
    governor::middleware::NoOpMiddleware, // re-exported `governor` module
};

/// Build the global rate limiter config.
///
/// - 3 token every second (≈ 1 RPS sustained)
/// - Burst size = 20
// src/middleware/rate_limiter.rs
pub fn rate_limiter_config() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware> {
    GovernorConfigBuilder::default()
        .seconds_per_request(3)
        .burst_size(20)
        .finish()
        .expect("valid governor config")
}
