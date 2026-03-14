use axum::{
    extract::{Extension, FromRequestParts, Request, State},
    http::header,
    http::request::Parts,
    middleware::Next,
    response::Response,
    RequestPartsExt as _,
};

use crate::{
    error::AppError,
    models::{AuthToken, CurrentUser},
    proto::auth::ValidateTokenRequest,
    proto::common::UserRole,
    AppState,
};

/// Middleware to enforce that the request has a valid JWT access token and to extract the current user information from it.
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer_token(request.headers()).inspect_err(
        |error| tracing::warn!(message = %error.message, "authorization header rejected"),
    )?;

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

    let current_user = CurrentUser::from(response);
    let auth_token = AuthToken {
        access_token: token,
    };

    // Insert for downstream handlers to access
    // the current user and auth token if needed
    request.extensions_mut().insert(current_user);
    request.extensions_mut().insert(auth_token);

    Ok(next.run(request).await)
}

/// Middleware to enforce that the current user has one of the specified roles.
pub fn require_roles(
    allowed_roles: &'static [UserRole],
) -> axum::middleware::FromExtractorLayer<RequireRoles, AllowedRoles> {
    axum::middleware::from_extractor_with_state(AllowedRoles(allowed_roles))
}

// Extractor to check if the current user's role is in the allowed roles list.
// Don't ask me why I'm using an extractor for this instead of a middleware,
// but it was the only way I could figure out how to pass the allowed roles list
// to the authorization logic without using global state.
//
// trying with closure that captures the allowed roles just a pain
// fighting with the generic types and lifetimes of the axum middleware system.
//
// Took me 3 hours fighting with the axum middleware system
// And this is the best I could come up with.
// If you have a better way to do this, feel free to change it.
#[derive(Clone, Copy)]
pub struct AllowedRoles(pub &'static [UserRole]);

pub struct RequireRoles;

impl FromRequestParts<AllowedRoles> for RequireRoles {
    type Rejection = crate::error::AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        allowed: &AllowedRoles,
    ) -> Result<Self, Self::Rejection> {
        let user = parts
            .extract::<Extension<CurrentUser>>()
            .await
            .map_err(|_| AppError::unauthorized("missing auth token"))?;

        if !allowed.0.contains(&user.role) {
            tracing::warn!(
                required_roles = ?allowed.0,
                user_role = ?user.role,
                "user role does not have permission to access this resource"
            );
            return Err(AppError::forbidden(
                "user role does not have permission to access this resource",
            ));
        }

        tracing::debug!(user_role = ?user.role, "user role authorized to access this resource");

        Ok(Self)
    }
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
