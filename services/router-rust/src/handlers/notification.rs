use axum::{
    extract::{Path, State},
    Extension, Json,
};

use crate::{
    error::AppError,
    models::{AckDto, CreateNotificationBody, CurrentUser, NotificationDto},
    proto::notification::{CreateNotificationRequest, ListNotificationsRequest, MarkAsReadRequest},
    AppState,
};

#[utoipa::path(
    get,
    path = "/api/notifications",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Notifications for current user", body = [NotificationDto]),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Notification service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Notification"
)]
pub async fn list_notifications(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Vec<NotificationDto>>, AppError> {
    let response = state
        .notification_client()
        .list_notifications(ListNotificationsRequest {
            user_id: current_user.user_id,
        })
        .await?
        .into_inner();

    let notifications = response
        .notifications
        .into_iter()
        .map(NotificationDto::from)
        .collect();

    Ok(Json(notifications))
}

#[utoipa::path(
    post,
    path = "/api/notifications",
    security(("bearer_auth" = [])),
    request_body = CreateNotificationBody,
    responses(
        (status = 200, description = "Notification created", body = NotificationDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Notification service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Notification"
)]
pub async fn create_notification(
    State(state): State<AppState>,
    Json(body): Json<CreateNotificationBody>,
) -> Result<Json<NotificationDto>, AppError> {
    let response = state
        .notification_client()
        .create_notification(CreateNotificationRequest {
            user_id: body.user_id,
            message: body.message,
        })
        .await?
        .into_inner();

    Ok(Json(NotificationDto::from(response)))
}

#[utoipa::path(
    post,
    path = "/api/notifications/{id}/read",
    security(("bearer_auth" = [])),
    params(
        ("id" = String, Path, description = "Notification id")
    ),
    responses(
        (status = 200, description = "Notification marked as read", body = AckDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Notification service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Notification"
)]
pub async fn mark_notification_read(
    State(state): State<AppState>,
    Path(notification_id): Path<String>,
) -> Result<Json<AckDto>, AppError> {
    let response = state
        .notification_client()
        .mark_as_read(MarkAsReadRequest { notification_id })
        .await?
        .into_inner();

    Ok(Json(AckDto::from(response)))
}
