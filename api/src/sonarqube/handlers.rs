use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, error};

use crate::sonarqube::SonarQubeService;

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub project_key: String,
    #[serde(default)]
    pub visibility: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetIssuesRequest {
    pub project_key: String,
    #[serde(default)]
    pub severities: Option<String>,
    #[serde(default)]
    pub types: Option<String>,
    #[serde(default)]
    pub statuses: Option<String>,
    #[serde(default)]
    pub created_after: Option<String>,
    #[serde(default)]
    pub created_before: Option<String>,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Create a new SonarQube project
pub async fn create_project(
    req: web::Json<CreateProjectRequest>,
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    info!("Creating SonarQube project: {}", req.project_key);

    match sonar_service
        .create_project(
            req.name.clone(),
            req.project_key.clone(),
            req.visibility.clone(),
        )
        .await
    {
        Ok(response) => {
            info!("Successfully created project: {}", req.project_key);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to create project {}: {}", req.project_key, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to create project: {}", e),
            )))
        }
    }
}

/// Get issues for a SonarQube project
pub async fn get_project_issues(
    req: web::Json<GetIssuesRequest>,
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    info!("Fetching issues for project: {}", req.project_key);

    let options = crate::sonarqube::service::IssueQueryOptions {
        severities: req.severities.clone(),
        types: req.types.clone(),
        statuses: req.statuses.clone(),
        created_after: req.created_after.clone(),
        created_before: req.created_before.clone(),
        page: req.page,
        page_size: req.page_size,
    };

    match sonar_service
        .get_project_issues(req.project_key.clone(), Some(options))
        .await
    {
        Ok(response) => {
            info!("Successfully retrieved {} issues for project: {}", response.paging.total, req.project_key);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to get issues for project {}: {}", req.project_key, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to get issues: {}", e),
            )))
        }
    }
}

/// Get all issues for a SonarQube project (with automatic pagination)
pub async fn get_all_project_issues(
    req: web::Json<GetIssuesRequest>,
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    info!("Fetching all issues for project: {}", req.project_key);

    let options = crate::sonarqube::service::IssueQueryOptions {
        severities: req.severities.clone(),
        types: req.types.clone(),
        statuses: req.statuses.clone(),
        created_after: req.created_after.clone(),
        created_before: req.created_before.clone(),
        page: Some(1),
        page_size: Some(500),
    };

    match sonar_service
        .get_all_project_issues(req.project_key.clone(), Some(options))
        .await
    {
        Ok(issues) => {
            info!("Successfully retrieved {} total issues for project: {}", issues.len(), req.project_key);
            Ok(HttpResponse::Ok().json(ApiResponse::success(issues)))
        }
        Err(e) => {
            error!("Failed to get all issues for project {}: {}", req.project_key, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to get all issues: {}", e),
            )))
        }
    }
}

/// Check if a project exists
pub async fn check_project_exists(
    path: web::Path<String>,
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    let project_key = path.into_inner();
    info!("Checking if project exists: {}", project_key);

    match sonar_service.project_exists(project_key.clone()).await {
        Ok(exists) => {
            let response = serde_json::json!({
                "project_key": project_key,
                "exists": exists
            });
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to check if project exists {}: {}", project_key, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to check project existence: {}", e),
            )))
        }
    }
}

/// Delete a SonarQube project
pub async fn delete_project(
    path: web::Path<String>,
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    let project_key = path.into_inner();
    info!("Deleting project: {}", project_key);

    match sonar_service.delete_project(project_key.clone()).await {
        Ok(_) => {
            info!("Successfully deleted project: {}", project_key);
            let response = serde_json::json!({
                "message": format!("Project {} deleted successfully", project_key)
            });
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to delete project {}: {}", project_key, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to delete project: {}", e),
            )))
        }
    }
}

/// Get project information
pub async fn get_project_info(
    path: web::Path<String>,
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    let project_key = path.into_inner();
    info!("Getting project info: {}", project_key);

    match sonar_service.get_project_info(project_key.clone()).await {
        Ok(info) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(info)))
        }
        Err(e) => {
            error!("Failed to get project info {}: {}", project_key, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to get project info: {}", e),
            )))
        }
    }
}

/// Health check endpoint for SonarQube connection
pub async fn health_check(
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    info!("Performing SonarQube health check");

    match sonar_service.health_check().await {
        Ok(is_healthy) => {
            if is_healthy {
                let response = serde_json::json!({
                    "status": "healthy",
                    "message": "SonarQube server is accessible"
                });
                Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
            } else {
                let response = serde_json::json!({
                    "status": "unhealthy",
                    "message": "SonarQube server is not accessible"
                });
                Ok(HttpResponse::ServiceUnavailable().json(ApiResponse::success(response)))
            }
        }
        Err(e) => {
            error!("Health check failed: {}", e);
            Ok(HttpResponse::ServiceUnavailable().json(ApiResponse::<()>::error(
                format!("Health check failed: {}", e),
            )))
        }
    }
}

/// Get SonarQube server version
pub async fn get_server_version(
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    info!("Getting SonarQube server version");

    match sonar_service.get_server_version().await {
        Ok(version) => {
            let response = serde_json::json!({
                "version": version
            });
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to get server version: {}", e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to get server version: {}", e),
            )))
        }
    }
}
