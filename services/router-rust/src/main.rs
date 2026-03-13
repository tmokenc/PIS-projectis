mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod proto;
mod state;

use config::Config;
pub use state::AppState;

use axum::{
    middleware as axum_middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use serde_json::json;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

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

async fn health() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);

        components.security_schemes.insert(
            "bearer_auth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::auth::get_user,
        handlers::auth::logout,
        handlers::subject::list_subjects,
        handlers::subject::get_subject,
        handlers::subject::register_subject,
        handlers::project::list_projects,
        handlers::project::get_project,
        handlers::project::register_project,
        handlers::project::add_team_member,
        handlers::project::remove_team_member,
        handlers::notification::list_notifications,
        handlers::notification::create_notification,
        handlers::notification::mark_notification_read
    ),
    components(
        schemas(
            models::ApiErrorResponse,
            models::AckDto,
            models::RegisterBody,
            models::LoginBody,
            models::UserDto,
            models::AuthDto,
            models::SubjectDto,
            models::ProjectDto,
            models::TeamDto,
            models::NotificationDto,
            models::RegisterSubjectBody,
            models::RegisterProjectBody,
            models::AddTeamMemberBody,
            models::RemoveTeamMemberBody,
            models::CreateNotificationBody
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication and authorization endpoints"),
        (name = "Subject", description = "Subject endpoints"),
        (name = "Project", description = "Project and team endpoints"),
        (name = "Notification", description = "Notification endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();
    init_telemetry(&config.otel_service_name, &config.otel_endpoint)?;

    let state = AppState::from_config(&config).await;

    let api_public = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login));

    let api_private = Router::new()
        .route("/auth/me", get(handlers::auth::get_user))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/subjects", get(handlers::subject::list_subjects))
        .route("/subjects/{id}", get(handlers::subject::get_subject))
        .route(
            "/subjects/register",
            post(handlers::subject::register_subject),
        )
        .route("/projects", get(handlers::project::list_projects))
        .route("/projects/{id}", get(handlers::project::get_project))
        .route(
            "/projects/register",
            post(handlers::project::register_project),
        )
        .route(
            "/teams/members",
            post(handlers::project::add_team_member).delete(handlers::project::remove_team_member),
        )
        .route(
            "/notifications",
            get(handlers::notification::list_notifications)
                .post(handlers::notification::create_notification),
        )
        .route(
            "/notifications/{id}/read",
            post(handlers::notification::mark_notification_read),
        )
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::require_auth,
        ));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_private.merge(api_public))
        .route("/health", get(health))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        );

    let listener = tokio::net::TcpListener::bind(&config.http_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
