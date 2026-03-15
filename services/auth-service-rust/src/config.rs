use std::env;

#[derive(Clone)]
pub struct Config {
    pub grpc_addr: String,
    pub db_path: String,
    pub surreal_namespace: String,
    pub surreal_database: String,
    pub jwt_secret: String,
    pub seed_demo_users: bool,
    pub otel_endpoint: String,
    pub otel_service_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            grpc_addr: env::var("AUTH_SERVICE_ADDR").unwrap_or_else(|_| "0.0.0.0:50051".into()),
            db_path: env::var("DB_PATH").unwrap_or_else(|_| "/data/auth-db".into()),
            surreal_namespace: env::var("SURREAL_NAMESPACE")
                .unwrap_or_else(|_| "university_auth".into()),
            surreal_database: env::var("SURREAL_DATABASE").unwrap_or_else(|_| "auth".into()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "change-me-in-production".into()),
            seed_demo_users: env::var("SEED_DEMO_USERS")
                .map(|value| {
                    matches!(
                        value.trim().to_ascii_lowercase().as_str(),
                        "1" | "true" | "yes" | "on"
                    )
                })
                .unwrap_or(true),
            otel_endpoint: env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://127.0.0.1:4317".into()),
            otel_service_name: env::var("OTEL_SERVICE_NAME")
                .unwrap_or_else(|_| "auth-service".into()),
        }
    }
}
