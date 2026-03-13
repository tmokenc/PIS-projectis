use axum::{extract::{Path, State}, Extension, Json};

use crate::{
    error::AppError,
    models::{AckDto, CurrentUser, RegisterSubjectBody, SubjectDto},
    proto::subject::{GetSubjectRequest, ListSubjectsRequest, RegisterStudentToSubjectRequest},
    AppState,
};

#[utoipa::path(
    get,
    path = "/api/subjects",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Available subjects", body = [SubjectDto]),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Subject service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Subject"
)]
pub async fn list_subjects(
    State(state): State<AppState>,
) -> Result<Json<Vec<SubjectDto>>, AppError> {
    let response = state
        .subject_client()
        .list_subjects(ListSubjectsRequest {})
        .await?
        .into_inner();

    let subjects = response.subjects.into_iter().map(SubjectDto::from).collect();
    Ok(Json(subjects))
}

#[utoipa::path(
    get,
    path = "/api/subjects/{id}",
    security(("bearer_auth" = [])),
    params(
        ("id" = String, Path, description = "Subject id")
    ),
    responses(
        (status = 200, description = "Subject detail", body = SubjectDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 404, description = "Subject not found", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Subject service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Subject"
)]
pub async fn get_subject(
    State(state): State<AppState>,
    Path(subject_id): Path<String>,
) -> Result<Json<SubjectDto>, AppError> {
    let response = state
        .subject_client()
        .get_subject(GetSubjectRequest { subject_id })
        .await?
        .into_inner();

    Ok(Json(SubjectDto::from(response)))
}

#[utoipa::path(
    post,
    path = "/api/subjects/register",
    security(("bearer_auth" = [])),
    request_body = RegisterSubjectBody,
    responses(
        (status = 200, description = "Student registered to subject", body = AckDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Subject service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Subject"
)]
pub async fn register_subject(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RegisterSubjectBody>,
) -> Result<Json<AckDto>, AppError> {
    let response = state
        .subject_client()
        .register_student_to_subject(RegisterStudentToSubjectRequest {
            subject_id: body.subject_id,
            student_id: current_user.user_id,
        })
        .await?
        .into_inner();

    Ok(Json(AckDto::from(response)))
}
