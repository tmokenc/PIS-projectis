use axum::{extract::State, Extension, Json};

use crate::{
    error::AppError,
    models::{parse_role, AuthDto, AuthToken, UserDto},
    proto::auth::{GetUserRequest, LoginRequest, LogoutRequest, RegisterRequest},
    AppState,
};

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = crate::models::RegisterBody,
    responses(
        (status = 200, description = "User registered successfully", body = AuthDto),
        (status = 400, description = "Invalid request", body = crate::models::ApiErrorResponse),
        (status = 409, description = "User already exists", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Auth service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<crate::models::RegisterBody>,
) -> Result<Json<AuthDto>, AppError> {
    let request = RegisterRequest {
        firstname: body.firstname,
        lastname: body.lastname,
        email: body.email,
        password: body.password,
        role: parse_role(&body.role) as i32,
    };

    let response = state.auth_client().register(request).await?.into_inner();
    Ok(Json(AuthDto::from(response)))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = crate::models::LoginBody,
    responses(
        (status = 200, description = "User logged in successfully", body = AuthDto),
        (status = 401, description = "Invalid credentials", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Auth service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<crate::models::LoginBody>,
) -> Result<Json<AuthDto>, AppError> {
    println!("Login attempt for email: {}", &body.email);
    let request = LoginRequest {
        email: body.email,
        password: body.password,
    };

    let response = state.auth_client().login(request).await?.into_inner();
    println!("Login successful");
    Ok(Json(AuthDto::from(response)))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Current user information", body = UserDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 404, description = "User not found", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Auth service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn get_user(
    State(state): State<AppState>,
    Extension(current_user): Extension<crate::models::CurrentUser>,
) -> Result<Json<UserDto>, AppError> {
    let response = state
        .auth_client()
        .get_user(GetUserRequest {
            user_id: current_user.user_id,
        })
        .await?
        .into_inner();

    Ok(Json(UserDto::from(response)))
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Jwt revoked", body = crate::models::AckDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Auth service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn logout(
    State(state): State<AppState>,
    Extension(token): Extension<AuthToken>,
) -> Result<Json<crate::models::AckDto>, AppError> {
    let response = state
        .auth_client()
        .logout(LogoutRequest {
            access_token: token.access_token,
        })
        .await?
        .into_inner();

    Ok(Json(crate::models::AckDto::from(response)))
}
