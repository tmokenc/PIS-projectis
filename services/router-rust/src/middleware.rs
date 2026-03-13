use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};

use crate::{
    error::AppError,
    models::{AuthToken, CurrentUser},
    proto::auth::ValidateTokenRequest,
    AppState,
};

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer_token(request.headers())
        .inspect_err(|error| tracing::warn!(message = %error.message, "authorization header rejected"))?;

    let response = state
        .auth_client()
        .validate_token(ValidateTokenRequest {
            access_token: token.clone(),
        })
        .await
        .inspect_err(|status| tracing::warn!(code = ?status.code(), message = status.message(), "token validation call failed"))?
        .into_inner();

    if !response.valid {
        tracing::warn!("invalid or expired jwt");
        return Err(AppError::unauthorized("invalid or expired jwt"));
    }

    let current_user = CurrentUser {
        user_id: response.user_id,
        email: response.email,
        role: crate::models::role_to_string(response.role),
    };

    request.extensions_mut().insert(current_user);
    request.extensions_mut().insert(AuthToken { access_token: token });

    Ok(next.run(request).await)
}

fn extract_bearer_token(headers: &axum::http::HeaderMap) -> Result<String, AppError> {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("missing authorization header"))?
        .to_str()
        .map_err(|_| AppError::unauthorized("invalid authorization header"))?;

    let token = authorization
        .strip_prefix("Bearer ")
        .or_else(|| authorization.strip_prefix("bearer "))
        .ok_or_else(|| AppError::unauthorized("authorization header must use bearer token"))?;

    if token.trim().is_empty() {
        return Err(AppError::unauthorized("authorization token is empty"));
    }

    Ok(token.to_string())
}
