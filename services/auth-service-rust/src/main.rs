use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::{
    engine::local::{Db as LocalDb, SurrealKv},
    RecordId, Surreal,
};
use tonic::{transport::Server, Request, Response, Status};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub mod auth {
    tonic::include_proto!("auth");
}
pub mod common {
    tonic::include_proto!("common");
}

use auth::auth_service_server::{AuthService, AuthServiceServer};
use auth::{
    AuthResponse, GetUserRequest, LoginRequest, LogoutRequest, RegisterRequest, User,
    ValidateTokenRequest, ValidateTokenResponse,
};
use common::{Ack, UserRole};

#[derive(Clone)]
struct Config {
    grpc_addr: String,
    db_path: String,
    surreal_namespace: String,
    surreal_database: String,
    jwt_secret: String,
    seed_demo_users: bool,
    otel_endpoint: String,
    otel_service_name: String,
}

impl Config {
    fn from_env() -> Self {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserRecord {
    id: RecordId,
    firstname: String,
    lastname: String,
    email: String,
    password_hash: String,
    role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewUserRecord {
    firstname: String,
    lastname: String,
    email: String,
    password_hash: String,
    role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RevokedTokenRecord {
    id: String,
    token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    role: String,
    exp: usize,
}

type Db = Surreal<LocalDb>;

#[derive(Clone)]
struct JwtKeys {
    secret: String,
}

#[derive(Clone)]
struct AuthGrpc {
    db: Db,
    jwt_keys: JwtKeys,
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

fn load_jwt_keys(config: &Config) -> anyhow::Result<JwtKeys> {
    Ok(JwtKeys {
        secret: config.jwt_secret.clone(),
    })
}

async fn connect_db(config: &Config) -> anyhow::Result<Db> {
    let db = Surreal::new::<SurrealKv>(config.db_path.clone()).await?;
    db.use_ns(&config.surreal_namespace)
        .use_db(&config.surreal_database)
        .await?;
    Ok(db)
}

async fn find_user_by_email(db: &Db, email: &str) -> anyhow::Result<Option<UserRecord>> {
    let mut result = db
        .query("SELECT * FROM user WHERE email = $email LIMIT 1;")
        .bind(("email", email.to_string()))
        .await?;
    let users: Vec<UserRecord> = result.take(0)?;
    Ok(users.into_iter().next())
}

async fn find_user_by_id(db: &Db, user_id: &str) -> anyhow::Result<Option<UserRecord>> {
    let user: Option<UserRecord> = db.select(("user", user_id)).await?;
    Ok(user)
}

async fn insert_user(db: &Db, user: &NewUserRecord) -> anyhow::Result<UserRecord> {
    let created: Option<UserRecord> = db.create("user").content(user.clone()).await?;
    created.ok_or_else(|| anyhow::anyhow!("failed to create user"))
}

async fn revoke_token(db: &Db, token: &str) -> anyhow::Result<()> {
    let rec = RevokedTokenRecord {
        id: uuid::Uuid::new_v4().to_string(),
        token: token.to_string(),
    };
    let _: Option<RevokedTokenRecord> = db
        .create(("revoked_token", rec.id.clone()))
        .content(rec)
        .await?;
    Ok(())
}

async fn is_token_revoked(db: &Db, token: &str) -> anyhow::Result<bool> {
    let mut result = db
        .query("SELECT * FROM revoked_token WHERE token = $token LIMIT 1;")
        .bind(("token", token.to_string()))
        .await?;
    let rows: Vec<RevokedTokenRecord> = result.take(0)?;
    Ok(!rows.is_empty())
}

fn create_token(
    user_id: &RecordId,
    email: &str,
    role: &str,
    jwt_secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp: (Utc::now() + Duration::hours(12)).timestamp() as usize,
    };

    let mut header = Header::new(Algorithm::HS256);
    header.typ = Some("JWT".into());

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}

fn decode_token(token: &str, jwt_secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(decoded.claims)
}

fn role_to_proto(role: &str) -> i32 {
    match role {
        "student" => UserRole::Student as i32,
        "teacher" => UserRole::Teacher as i32,
        "admin" => UserRole::Admin as i32,
        _ => UserRole::Unspecified as i32,
    }
}

fn role_from_proto(role: i32) -> Result<&'static str, Status> {
    match UserRole::try_from(role).map_err(|_| Status::invalid_argument("invalid role"))? {
        UserRole::Student => Ok("student"),
        UserRole::Teacher => Ok("teacher"),
        UserRole::Admin => Ok("admin"),
        UserRole::Unspecified => Err(Status::invalid_argument("role must be specified")),
    }
}

impl From<UserRecord> for User {
    fn from(user: UserRecord) -> Self {
        User {
            id: user.id.to_string(),
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            role: role_to_proto(&user.role),
        }
    }
}

fn internal<E: std::fmt::Display>(err: E) -> Status {
    Status::internal(err.to_string())
}

async fn seed_demo_users(db: &Db) -> anyhow::Result<()> {
    let demo_users = [
        (
            "student@example.com",
            "student123",
            "student",
            "Demo",
            "Student",
        ),
        (
            "teacher@example.com",
            "teacher123",
            "teacher",
            "Demo",
            "Teacher",
        ),
        ("admin@example.com", "admin123", "admin", "Demo", "Admin"),
    ];

    for (email, password, role, firstname, lastname) in demo_users {
        if find_user_by_email(db, email).await?.is_some() {
            continue;
        }

        let user = NewUserRecord {
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            email: email.to_string(),
            password_hash: hash(password, DEFAULT_COST)?,
            role: role.to_string(),
        };

        insert_user(db, &user).await?;
        tracing::info!(email, role, "seeded demo user");
    }

    Ok(())
}

#[tonic::async_trait]
impl AuthService for AuthGrpc {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        if find_user_by_email(&self.db, &req.email)
            .await
            .map_err(internal)?
            .is_some()
        {
            return Err(Status::already_exists("user already exists"));
        }

        let role = role_from_proto(req.role)?.to_string();
        let user = NewUserRecord {
            firstname: req.firstname,
            lastname: req.lastname,
            email: req.email,
            password_hash: hash(req.password, DEFAULT_COST).map_err(internal)?,
            role,
        };

        let created = insert_user(&self.db, &user).await.map_err(internal)?;
        let token = create_token(
            &created.id,
            &created.email,
            &created.role,
            &self.jwt_keys.secret,
        )
        .map_err(internal)?;

        Ok(Response::new(AuthResponse {
            access_token: token,
            user: Some(created.into()),
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let user = find_user_by_email(&self.db, &req.email)
            .await
            .map_err(internal)?
            .ok_or_else(|| Status::unauthenticated("invalid credentials"))?;
        if !verify(req.password, &user.password_hash).map_err(internal)? {
            return Err(Status::unauthenticated("invalid credentials"));
        }
        let token = create_token(&user.id, &user.email, &user.role, &self.jwt_keys.secret)
            .map_err(internal)?;
        Ok(Response::new(AuthResponse {
            access_token: token,
            user: Some(user.into()),
        }))
    }

    async fn validate_token(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        let req = request.into_inner();
        if is_token_revoked(&self.db, &req.access_token)
            .await
            .map_err(internal)?
        {
            return Ok(Response::new(ValidateTokenResponse {
                valid: false,
                user_id: String::new(),
                email: String::new(),
                role: UserRole::Unspecified as i32,
            }));
        }
        match decode_token(&req.access_token, &self.jwt_keys.secret) {
            Ok(claims) => Ok(Response::new(ValidateTokenResponse {
                valid: true,
                user_id: claims.sub,
                email: claims.email,
                role: role_to_proto(&claims.role),
            })),
            Err(_) => Ok(Response::new(ValidateTokenResponse {
                valid: false,
                user_id: String::new(),
                email: String::new(),
                role: UserRole::Unspecified as i32,
            })),
        }
    }

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let req = request.into_inner();
        let user = find_user_by_id(&self.db, &req.user_id)
            .await
            .map_err(internal)?
            .ok_or_else(|| Status::not_found("user not found"))?;
        Ok(Response::new(user.into()))
    }

    async fn logout(&self, request: Request<LogoutRequest>) -> Result<Response<Ack>, Status> {
        revoke_token(&self.db, &request.into_inner().access_token)
            .await
            .map_err(internal)?;
        Ok(Response::new(Ack {
            success: true,
            message: "token revoked".into(),
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    init_telemetry(&config.otel_service_name, &config.otel_endpoint)?;

    let db = connect_db(&config).await?;

    if config.seed_demo_users {
        seed_demo_users(&db).await?;
    }

    let jwt_keys = load_jwt_keys(&config)?;
    let addr = config.grpc_addr.parse()?;

    Server::builder()
        .add_service(AuthServiceServer::new(AuthGrpc { db, jwt_keys }))
        .serve(addr)
        .await?;

    Ok(())
}
