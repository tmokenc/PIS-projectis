use crate::db::Db;
use crate::models;
use crate::models::{Claims, JwtKeys, NewUserRecord};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use bcrypt::{hash, verify, DEFAULT_COST};
use tonic::{Request, Response, Status};

pub mod auth {
    tonic::include_proto!("auth");
}

pub mod common {
    #![allow(unused)]
    tonic::include_proto!("common");
}

use auth::auth_service_server::AuthService;
use auth::{
    AuthResponse, GetUserRequest, LoginRequest, LogoutRequest, RegisterRequest, User,
    ValidateTokenRequest, ValidateTokenResponse,
};
use common::{Ack, UserRole};

#[derive(Clone)]
pub struct AuthGrpc {
    db: Db,
    jwt_keys: JwtKeys,
}

impl AuthGrpc {
    pub fn new(db: Db, jwt_keys: JwtKeys) -> Self {
        Self { db, jwt_keys }
    }
}

fn internal<E: std::fmt::Display>(err: E) -> Status {
    Status::internal(err.to_string())
}

#[tonic::async_trait]
impl AuthService for AuthGrpc {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        if self
            .db
            .find_user_by_email(&req.email)
            .await
            .map_err(internal)?
            .is_some()
        {
            return Err(Status::already_exists("user already exists"));
        }

        let role = models::role_from_proto(req.role)?.to_owned();
        let user = NewUserRecord {
            firstname: req.firstname,
            lastname: req.lastname,
            email: req.email,
            password_hash: hash(req.password, DEFAULT_COST).map_err(internal)?,
            role,
        };

        let created = self.db.insert_user(&user).await.map_err(internal)?;
        let token =
            create_token(&created.id, &created.role, &self.jwt_keys.secret).map_err(internal)?;

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
        let user = self
            .db
            .find_user_by_email(&req.email)
            .await
            .map_err(internal)?
            .ok_or_else(|| Status::unauthenticated("invalid credentials"))?;

        if !verify(req.password, &user.password_hash).map_err(internal)? {
            return Err(Status::unauthenticated("invalid credentials"));
        }

        let token = create_token(&user.id, &user.role, &self.jwt_keys.secret).map_err(internal)?;

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
        if self
            .db
            .is_token_revoked(&req.access_token)
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
            Ok(claims) => {
                // double check
                let user = self
                    .db
                    .find_user_by_id(&claims.sub)
                    .await
                    .map_err(internal)?
                    .ok_or_else(|| Status::unauthenticated("invalid token: user does not exist"))?;

                Ok(Response::new(ValidateTokenResponse {
                    valid: true,
                    user_id: user.id.to_string(),
                    email: user.email,
                    role: models::role_to_proto(&user.role),
                }))
            }
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
        let user = self
            .db
            .find_user_by_id(&req.user_id)
            .await
            .map_err(internal)?
            .ok_or_else(|| Status::not_found("user not found"))?;

        Ok(Response::new(user.into()))
    }

    async fn logout(&self, request: Request<LogoutRequest>) -> Result<Response<Ack>, Status> {
        let _ = self
            .db
            .revoke_token(&request.into_inner().access_token)
            .await
            .map_err(internal)?;

        Ok(Response::new(Ack {
            success: true,
            message: "token revoked".into(),
        }))
    }
}

// ====== Helper functions ======

fn create_token(
    record_id: &surrealdb::RecordId,
    role: &str,
    jwt_secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let userid = record_id.key().to_string().trim_matches('`').to_string();

    let claims = Claims {
        sub: userid,
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
