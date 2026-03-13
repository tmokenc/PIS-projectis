use axum::{extract::{Path, State}, Json};

use crate::{
    error::AppError,
    models::{AddTeamMemberBody, ProjectDto, RegisterProjectBody, RemoveTeamMemberBody, TeamDto},
    proto::project::{
        AddTeamMemberRequest, GetProjectRequest, ListProjectsRequest, RegisterTeamRequest,
        RemoveTeamMemberRequest,
    },
    AppState,
};

#[utoipa::path(
    get,
    path = "/api/projects",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Available projects", body = [ProjectDto]),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Project service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Project"
)]
pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectDto>>, AppError> {
    let response = state
        .project_client()
        .list_projects(ListProjectsRequest {})
        .await?
        .into_inner();

    let projects = response.projects.into_iter().map(ProjectDto::from).collect();
    Ok(Json(projects))
}

#[utoipa::path(
    get,
    path = "/api/projects/{id}",
    security(("bearer_auth" = [])),
    params(
        ("id" = String, Path, description = "Project id")
    ),
    responses(
        (status = 200, description = "Project detail", body = ProjectDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 404, description = "Project not found", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Project service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Project"
)]
pub async fn get_project(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<ProjectDto>, AppError> {
    let response = state
        .project_client()
        .get_project(GetProjectRequest { project_id })
        .await?
        .into_inner();

    Ok(Json(ProjectDto::from(response)))
}

#[utoipa::path(
    post,
    path = "/api/projects/register",
    security(("bearer_auth" = [])),
    request_body = RegisterProjectBody,
    responses(
        (status = 200, description = "Team created for selected project", body = TeamDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Project service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Project"
)]
pub async fn register_project(
    State(state): State<AppState>,
    axum::Extension(current_user): axum::Extension<crate::models::CurrentUser>,
    Json(body): Json<RegisterProjectBody>,
) -> Result<Json<TeamDto>, AppError> {
    let response = state
        .project_client()
        .register_team(RegisterTeamRequest {
            project_id: body.project_id,
            creator_student_id: current_user.user_id,
        })
        .await?
        .into_inner();

    Ok(Json(TeamDto::from(response)))
}

#[utoipa::path(
    post,
    path = "/api/teams/members",
    security(("bearer_auth" = [])),
    request_body = AddTeamMemberBody,
    responses(
        (status = 200, description = "Team member added", body = TeamDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Project service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Project"
)]
pub async fn add_team_member(
    State(state): State<AppState>,
    Json(body): Json<AddTeamMemberBody>,
) -> Result<Json<TeamDto>, AppError> {
    let response = state
        .project_client()
        .add_team_member(AddTeamMemberRequest {
            team_id: body.team_id,
            student_id: body.student_id,
        })
        .await?
        .into_inner();

    Ok(Json(TeamDto::from(response)))
}

#[utoipa::path(
    delete,
    path = "/api/teams/members",
    security(("bearer_auth" = [])),
    request_body = RemoveTeamMemberBody,
    responses(
        (status = 200, description = "Team member removed", body = TeamDto),
        (status = 401, description = "Missing or invalid jwt", body = crate::models::ApiErrorResponse),
        (status = 502, description = "Project service failure", body = crate::models::ApiErrorResponse)
    ),
    tag = "Project"
)]
pub async fn remove_team_member(
    State(state): State<AppState>,
    Json(body): Json<RemoveTeamMemberBody>,
) -> Result<Json<TeamDto>, AppError> {
    let response = state
        .project_client()
        .remove_team_member(RemoveTeamMemberRequest {
            team_id: body.team_id,
            student_id: body.student_id,
        })
        .await?
        .into_inner();

    Ok(Json(TeamDto::from(response)))
}
