use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::proto::{common::Ack, notification::Notification, project::Project, project::Team, subject::Subject};

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AckDto {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterBody {
    #[schema(example = "Nguyen")]
    pub firstname: String,
    #[schema(example = "Le Duy")]
    pub lastname: String,
    #[schema(example = "duy@example.com")]
    pub email: String,
    #[schema(example = "strong-password")]
    pub password: String,
    #[schema(example = "student")]
    pub role: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginBody {
    #[schema(example = "duy@example.com")]
    pub email: String,
    #[schema(example = "strong-password")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthDto {
    pub access_token: String,
    pub user: Option<UserDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubjectDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub abbreviation: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub teacher_id: String,
    pub max_students_per_team: u32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub subject_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TeamDto {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub leader_student_id: String,
    pub student_ids: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NotificationDto {
    pub id: String,
    pub user_id: String,
    pub message: String,
    pub date: Option<String>,
    pub read: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterSubjectBody {
    #[schema(example = "subject:oop")]
    pub subject_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterProjectBody {
    #[schema(example = "project:distributed-tracing")]
    pub project_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddTeamMemberBody {
    #[schema(example = "team:alpha")]
    pub team_id: String,
    #[schema(example = "student:123")]
    pub student_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RemoveTeamMemberBody {
    #[schema(example = "team:alpha")]
    pub team_id: String,
    #[schema(example = "student:123")]
    pub student_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNotificationBody {
    #[schema(example = "user:123")]
    pub user_id: String,
    #[schema(example = "Project registration deadline is tomorrow.")]
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct AuthToken {
    pub access_token: String,
}

impl From<Ack> for AckDto {
    fn from(value: Ack) -> Self {
        Self {
            success: value.success,
            message: value.message,
        }
    }
}

impl From<crate::proto::auth::User> for UserDto {
    fn from(user: crate::proto::auth::User) -> Self {
        Self {
            id: user.id,
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            role: role_to_string(user.role),
        }
    }
}

impl From<crate::proto::auth::AuthResponse> for AuthDto {
    fn from(value: crate::proto::auth::AuthResponse) -> Self {
        Self {
            access_token: value.access_token,
            user: value.user.map(UserDto::from),
        }
    }
}

impl From<Subject> for SubjectDto {
    fn from(subject: Subject) -> Self {
        Self {
            id: subject.id,
            name: subject.name,
            description: subject.description,
            abbreviation: subject.abbreviation,
        }
    }
}

impl From<Project> for ProjectDto {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            title: project.title,
            description: project.description,
            teacher_id: project.teacher_id,
            max_students_per_team: project.max_students_per_team,
            start_date: timestamp_to_rfc3339(project.start_date),
            end_date: timestamp_to_rfc3339(project.end_date),
            subject_id: project.subject_id,
        }
    }
}

impl From<Team> for TeamDto {
    fn from(team: Team) -> Self {
        Self {
            id: team.id,
            project_id: team.project_id,
            name: team.name,
            leader_student_id: team.leader_student_id,
            student_ids: team.student_ids,
        }
    }
}

impl From<Notification> for NotificationDto {
    fn from(notification: Notification) -> Self {
        Self {
            id: notification.id,
            user_id: notification.user_id,
            message: notification.message,
            date: timestamp_to_rfc3339(notification.date),
            read: notification.read,
        }
    }
}

pub fn role_to_string(role: i32) -> String {
    use crate::proto::common::UserRole;

    match UserRole::try_from(role).unwrap_or(UserRole::Unspecified) {
        UserRole::Admin => "admin".to_string(),
        UserRole::Teacher => "teacher".to_string(),
        UserRole::Student => "student".to_string(),
        UserRole::Unspecified => "unspecified".to_string(),
    }
}

pub fn parse_role(role: &str) -> crate::proto::common::UserRole {
    use crate::proto::common::UserRole;

    match role.to_lowercase().as_str() {
        "admin" => UserRole::Admin,
        "teacher" => UserRole::Teacher,
        "student" => UserRole::Student,
        _ => UserRole::Student,
    }
}

fn timestamp_to_rfc3339(timestamp: Option<Timestamp>) -> Option<String> {
    let timestamp = timestamp?;
    let date = DateTime::<Utc>::from_timestamp(timestamp.seconds, timestamp.nanos as u32)?;
    Some(date.to_rfc3339())
}
