use chrono::Utc;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tokio::signal;
use tonic::{transport::Server, Request, Response, Status};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub mod common {
    tonic::include_proto!("common");
}
pub mod notification {
    tonic::include_proto!("notification");
}

use common::Ack;
use notification::notification_service_server::{NotificationService, NotificationServiceServer};
use notification::{
    CreateNotificationRequest, ListNotificationsRequest, ListNotificationsResponse,
    MarkAsReadRequest, Notification,
};

#[derive(Clone)]
struct Config {
    grpc_addr: String,
    db_path: String,
    otel_endpoint: String,
    otel_service_name: String,
}

impl Config {
    fn from_env() -> Self {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationRecord {
    id: String,
    user_id: String,
    message: String,
    date: chrono::DateTime<Utc>,
    read: bool,
}

#[derive(Clone)]
struct NotificationStore {
    db: Database,
    notifications: Keyspace,
    user_index: Keyspace,
}

impl NotificationStore {
    fn open(path: &str) -> anyhow::Result<Self> {
        let db = Database::builder(path).open()?;
        let notifications = db.keyspace("notifications", KeyspaceCreateOptions::default)?;
        let user_index = db.keyspace("user_notifications", KeyspaceCreateOptions::default)?;
        Ok(Self {
            db,
            notifications,
            user_index,
        })
    }

    fn user_index_key(user_id: &str, date: chrono::DateTime<Utc>, notification_id: &str) -> String {
        let millis = date.timestamp_millis().max(0) as u64;
        let reverse_millis = u64::MAX - millis;
        format!("{}:{:020}:{}", user_id, reverse_millis, notification_id)
    }

    fn save_notification(&self, record: &NotificationRecord) -> anyhow::Result<()> {
        let value = serde_json::to_vec(record)?;
        self.notifications.insert(record.id.as_bytes(), value)?;
        let index_key = Self::user_index_key(&record.user_id, record.date, &record.id);
        self.user_index
            .insert(index_key.as_bytes(), record.id.as_bytes())?;
        self.db.persist(PersistMode::SyncAll)?;
        Ok(())
    }

    fn get_notification(&self, id: &str) -> anyhow::Result<Option<NotificationRecord>> {
        let Some(bytes) = self.notifications.get(id.as_bytes())? else {
            return Ok(None);
        };
        Ok(Some(serde_json::from_slice(&bytes)?))
    }

    fn list_notifications(&self, user_id: &str) -> anyhow::Result<Vec<NotificationRecord>> {
        let mut rows = Vec::new();
        let prefix = format!("{}:", user_id);
        for entry in self.user_index.prefix(prefix.as_bytes()) {
            let value = entry.value()?;
            let notification_id = std::str::from_utf8(&value)?;
            if let Some(record) = self.get_notification(notification_id)? {
                rows.push(record);
            }
        }
        Ok(rows)
    }

    fn mark_as_read(&self, notification_id: &str) -> anyhow::Result<()> {
        let Some(mut record) = self.get_notification(notification_id)? else {
            return Ok(());
        };
        record.read = true;
        let value = serde_json::to_vec(&record)?;
        self.notifications.insert(record.id.as_bytes(), value)?;
        self.db.persist(PersistMode::SyncAll)?;
        Ok(())
    }
}

#[derive(Clone)]
struct NotificationGrpc {
    store: Arc<NotificationStore>,
}

fn init_telemetry(service_name: &str, otlp_endpoint: &str) -> anyhow::Result<()> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()?;
    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(
            Resource::builder()
                .with_service_name(service_name.to_string())
                .build(),
        )
        .build();
    let tracer = provider.tracer(service_name.to_string());

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    opentelemetry::global::set_tracer_provider(provider);
    Ok(())
}

impl From<NotificationRecord> for Notification {
    fn from(rec: NotificationRecord) -> Self {
        Notification {
            id: rec.id,
            user_id: rec.user_id,
            message: rec.message,
            date: Some(Timestamp {
                seconds: rec.date.timestamp(),
                nanos: rec.date.timestamp_subsec_nanos() as i32,
            }),
            read: rec.read,
        }
    }
}

fn internal<E: std::fmt::Display>(err: E) -> Status {
    Status::internal(err.to_string())
}

#[tonic::async_trait]
impl NotificationService for NotificationGrpc {
    async fn create_notification(
        &self,
        request: Request<CreateNotificationRequest>,
    ) -> Result<Response<Notification>, Status> {
        let req = request.into_inner();
        let record = NotificationRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: req.user_id,
            message: req.message,
            date: Utc::now(),
            read: false,
        };

        self.store.save_notification(&record).map_err(internal)?;
        Ok(Response::new(record.into()))
    }

    async fn list_notifications(
        &self,
        request: Request<ListNotificationsRequest>,
    ) -> Result<Response<ListNotificationsResponse>, Status> {
        let rows = self
            .store
            .list_notifications(&request.into_inner().user_id)
            .map_err(internal)?;

        Ok(Response::new(ListNotificationsResponse {
            notifications: rows.into_iter().map(Notification::from).collect(),
        }))
    }

    async fn mark_as_read(
        &self,
        request: Request<MarkAsReadRequest>,
    ) -> Result<Response<Ack>, Status> {
        self.store
            .mark_as_read(&request.into_inner().notification_id)
            .map_err(internal)?;

        Ok(Response::new(Ack {
            success: true,
            message: "notification marked as read".into(),
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    init_telemetry(&config.otel_service_name, &config.otel_endpoint)?;

    let store = Arc::new(NotificationStore::open(&config.db_path)?);
    let addr = config.grpc_addr.parse()?;

    Server::builder()
        .add_service(NotificationServiceServer::new(NotificationGrpc { store }))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    Ok(())
}

// Samelessly taken from axum's graceful shutdown example:
// https://github.com/tokio-rs/axum/blob/da26db264f811e73485f1db1c134d374e8f99464/examples/graceful-shutdown/src/main.rs#L54
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
