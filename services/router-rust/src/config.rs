use std::env;

#[derive(Clone)]
pub struct Config {
    pub(crate) http_addr: String,
    pub(crate) auth_grpc_endpoint: String,
    pub(crate) notification_grpc_endpoint: String,
    pub(crate) subject_grpc_endpoint: String,
    pub(crate) project_grpc_endpoint: String,
    pub(crate) otel_endpoint: String,
    pub(crate) otel_service_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            http_addr: env::var("ROUTER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into()),
            auth_grpc_endpoint: env::var("AUTH_GRPC_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:50051".into()),
            notification_grpc_endpoint: env::var("NOTIFICATION_GRPC_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:50052".into()),
            subject_grpc_endpoint: env::var("SUBJECT_GRPC_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:50053".into()),
            project_grpc_endpoint: env::var("PROJECT_GRPC_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:50054".into()),
            otel_endpoint: env::var("OTEL_EXPORTER_OTLP_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:4317".into()),
            otel_service_name: env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "router".into()),
        }
    }
}


