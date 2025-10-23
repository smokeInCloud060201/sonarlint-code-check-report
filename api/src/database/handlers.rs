use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Serialize};
use std::sync::Arc;
use tracing::{info, error};
use uuid::Uuid;

use crate::database::ProjectService;

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

/// Get all projects from database
pub async fn get_all_projects(
    project_service: web::Data<Arc<ProjectService>>,
) -> ActixResult<HttpResponse> {
    info!("Fetching all projects from database");

    match project_service.get_all_active_projects().await {
        Ok(projects) => {
            info!("Successfully retrieved {} projects from database", projects.len());
            Ok(HttpResponse::Ok().json(ApiResponse::success(projects)))
        }
        Err(e) => {
            error!("Failed to get projects from database: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to get projects: {}", e),
            )))
        }
    }
}

/// Get project by ID
pub async fn get_project_by_id(
    path: web::Path<Uuid>,
    project_service: web::Data<Arc<ProjectService>>,
) -> ActixResult<HttpResponse> {
    let project_id = path.into_inner();
    info!("Fetching project by ID: {}", project_id);

    match project_service.find_by_id(project_id).await {
        Ok(Some(project)) => {
            info!("Successfully retrieved project: {}", project.sonarqube_key);
            Ok(HttpResponse::Ok().json(ApiResponse::success(project)))
        }
        Ok(None) => {
            info!("Project not found: {}", project_id);
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
                "Project not found".to_string(),
            )))
        }
        Err(e) => {
            error!("Failed to get project {}: {}", project_id, e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to get project: {}", e),
            )))
        }
    }
}

/// Get project by SonarQube key
pub async fn get_project_by_sonarqube_key(
    path: web::Path<String>,
    project_service: web::Data<Arc<ProjectService>>,
) -> ActixResult<HttpResponse> {
    let sonarqube_key = path.into_inner();
    info!("Fetching project by SonarQube key: {}", sonarqube_key);

    match project_service.find_by_sonarqube_key(&sonarqube_key).await {
        Ok(Some(project)) => {
            info!("Successfully retrieved project: {}", project.sonarqube_key);
            Ok(HttpResponse::Ok().json(ApiResponse::success(project)))
        }
        Ok(None) => {
            info!("Project not found: {}", sonarqube_key);
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
                "Project not found".to_string(),
            )))
        }
        Err(e) => {
            error!("Failed to get project {}: {}", sonarqube_key, e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to get project: {}", e),
            )))
        }
    }
}

/// Deactivate project (soft delete)
pub async fn deactivate_project(
    path: web::Path<Uuid>,
    project_service: web::Data<Arc<ProjectService>>,
) -> ActixResult<HttpResponse> {
    let project_id = path.into_inner();
    info!("Deactivating project: {}", project_id);

    match project_service.deactivate_project(project_id).await {
        Ok(project) => {
            info!("Successfully deactivated project: {}", project.sonarqube_key);
            Ok(HttpResponse::Ok().json(ApiResponse::success(project)))
        }
        Err(e) => {
            error!("Failed to deactivate project {}: {}", project_id, e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to deactivate project: {}", e),
            )))
        }
    }
}

/// Delete project (hard delete)
pub async fn delete_project_from_db(
    path: web::Path<Uuid>,
    project_service: web::Data<Arc<ProjectService>>,
) -> ActixResult<HttpResponse> {
    let project_id = path.into_inner();
    info!("Deleting project from database: {}", project_id);

    match project_service.delete_project(project_id).await {
        Ok(_) => {
            info!("Successfully deleted project: {}", project_id);
            let response = serde_json::json!({
                "message": format!("Project {} deleted successfully", project_id)
            });
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to delete project {}: {}", project_id, e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to delete project: {}", e),
            )))
        }
    }
}
