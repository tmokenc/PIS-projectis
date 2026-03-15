use std::env;

#[derive(Clone)]
pub struct Config {
    pub grpc_addr: String,
    pub db_path: String,
    pub otel_endpoint: String,
    pub otel_service_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            grpc_addr: env::var("NOTIFICATION_SERVICE_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:50052".into()),
            db_path: env::var("DB_PATH").unwrap_or_else(|_| "/data/notification-db".into()),
            otel_endpoint: env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://127.0.0.1:4317".into()),
            otel_service_name: env::var("OTEL_SERVICE_NAME")
                .unwrap_or_else(|_| "notification-service".into()),
        }
    }
}
