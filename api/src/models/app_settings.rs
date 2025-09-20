#[derive(Clone)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
    pub short_url_base: String,
    pub dynamodb_endpoint: String,
}

const ENV_HOST: &str = "HOST";
const ENV_PORT: &str = "PORT";
const ENV_SHORT_URL_BASE: &str = "SHORT_URL_BASE";
const ENV_DDB_ENDPOINT: &str = "DYNAMODB_ENDPOINT";

impl AppSettings {
    pub fn from_env_or_default() -> Self {
        let host = std::env::var(ENV_HOST).unwrap_or_else(|_| "127.0.0.1".into());

        let port: u16 = std::env::var(ENV_PORT)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000);

        let short_url_base =
            std::env::var(ENV_SHORT_URL_BASE).unwrap_or_else(|_| format!("http://{host}:{port}/"));

        let dynamodb_endpoint =
            std::env::var(ENV_DDB_ENDPOINT).unwrap_or_else(|_| "".into());

        Self {
            host,
            port,
            short_url_base,
            dynamodb_endpoint,
        }
    }
}
