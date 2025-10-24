use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, error, warn};

use crate::sonarqube::SonarQubeService;
use crate::database::TokenService;

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub project_key: String,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub project_folder_path: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    #[serde(default)]
    pub project_key: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<String>,
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
    token_service: web::Data<Arc<TokenService>>,
) -> ActixResult<HttpResponse> {
    info!("Creating SonarQube project: {} with folder path: {:?}", req.project_key, req.project_folder_path);

    // Step 1: Create project in SonarQube
    match sonar_service
        .create_project(
            req.name.clone(),
            req.project_key.clone(),
            req.visibility.clone(),
            req.project_folder_path.clone(),
        )
        .await
    {
        Ok(response) => {
            info!("Successfully created project in SonarQube: {}", req.project_key);
            
            // Step 2: Create project token in SonarQube
            let token_name = format!("{}-token", req.project_key);
            match sonar_service
                .generate_token(
                    token_name.clone(),
                    Some(req.project_key.clone()),
                    Some(format!("Token for project {}", req.name)),
                    None,
                )
                .await
            {
                Ok(token_response) => {
                    info!("Successfully generated token for project: {}", req.project_key);
                    
                    // Step 3: Save token to database
                    match token_service
                        .create_token(
                            token_response.name.clone(),
                            token_response.token.clone(),
                            Some(req.project_key.clone()),
                            Some("system".to_string()),
                            token_response.description.clone(),
                        )
                        .await
                    {
                        Ok(_) => {
                            info!("Successfully saved token to database for project: {}", req.project_key);
                            
                            // Return success response with both project and token info
                            let response_data = serde_json::json!({
                                "project": response.project,
                                "token": {
                                    "name": token_response.name,
                                    "token": token_response.token,
                                    "project_key": token_response.project_key
                                },
                                "project_folder_path": req.project_folder_path
                            });
                            
                            Ok(HttpResponse::Ok().json(ApiResponse::success(response_data)))
                        }
                        Err(e) => {
                            error!("Failed to save token to database for project {}: {}", req.project_key, e);
                            // Still return success for project creation, but warn about token save failure
                            let response_data = serde_json::json!({
                                "project": response.project,
                                "warning": "Project created but token save failed",
                                "project_folder_path": req.project_folder_path
                            });
                            Ok(HttpResponse::Ok().json(ApiResponse::success(response_data)))
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to generate token for project {}: {}", req.project_key, e);
                    // Return success for project creation, but warn about token generation failure
                    let response_data = serde_json::json!({
                        "project": response.project,
                        "warning": "Project created but token generation failed",
                        "project_folder_path": req.project_folder_path
                    });
                    Ok(HttpResponse::Ok().json(ApiResponse::success(response_data)))
                }
            }
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

/// Generate a new SonarQube token
pub async fn generate_token(
    req: web::Json<CreateTokenRequest>,
    sonar_service: web::Data<Arc<SonarQubeService>>,
    token_service: web::Data<Arc<TokenService>>,
) -> ActixResult<HttpResponse> {
    info!("Generating SonarQube token: {}", req.name);

    match sonar_service
        .generate_token(
            req.name.clone(),
            req.project_key.clone(),
            req.description.clone(),
            req.expires_at.clone(),
        )
        .await
    {
        Ok(response) => {
            // Save token to database
            match token_service
                .create_token(
                    response.name.clone(),
                    response.token.clone(),
                    response.project_key.clone(),
                    req.created_by.clone(),
                    response.description.clone(),
                )
                .await
            {
                Ok(db_token) => {
                    info!("Successfully saved token to database: {}", db_token.name);
                    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
                }
                Err(e) => {
                    error!("Failed to save token to database: {}", e);
                    // Still return the token even if database save fails
                    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
                }
            }
        }
        Err(e) => {
            error!("Failed to generate token {}: {}", req.name, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to generate token: {}", e),
            )))
        }
    }
}

/// List all SonarQube tokens
pub async fn list_tokens(
    sonar_service: web::Data<Arc<SonarQubeService>>,
) -> ActixResult<HttpResponse> {
    info!("Listing SonarQube tokens");

    match sonar_service.list_tokens().await {
        Ok(response) => {
            info!("Successfully retrieved {} tokens", response.tokens.len());
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to list tokens: {}", e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to list tokens: {}", e),
            )))
        }
    }
}

/// Revoke a SonarQube token
pub async fn revoke_token(
    path: web::Path<String>,
    sonar_service: web::Data<Arc<SonarQubeService>>,
    token_service: web::Data<Arc<TokenService>>,
) -> ActixResult<HttpResponse> {
    let token_name = path.into_inner();
    info!("Revoking SonarQube token: {}", token_name);

    // First revoke in SonarQube
    match sonar_service.revoke_token(token_name.clone()).await {
        Ok(_) => {
            // Then deactivate in database
            match token_service.get_token_by_name(&token_name).await {
                Ok(Some(token)) => {
                    match token_service.deactivate_token(token.id).await {
                        Ok(_) => {
                            info!("Successfully revoked and deactivated token: {}", token_name);
                            let response = serde_json::json!({
                                "message": format!("Token {} revoked successfully", token_name)
                            });
                            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
                        }
                        Err(e) => {
                            error!("Failed to deactivate token in database: {}", e);
                            // Token was revoked in SonarQube but not in database
                            let response = serde_json::json!({
                                "message": format!("Token {} revoked in SonarQube but failed to update database", token_name)
                            });
                            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
                        }
                    }
                }
                Ok(None) => {
                    warn!("Token not found in database: {}", token_name);
                    let response = serde_json::json!({
                        "message": format!("Token {} revoked in SonarQube but not found in database", token_name)
                    });
                    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
                }
                Err(e) => {
                    error!("Failed to find token in database: {}", e);
                    let response = serde_json::json!({
                        "message": format!("Token {} revoked in SonarQube but failed to find in database", token_name)
                    });
                    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
                }
            }
        }
        Err(e) => {
            error!("Failed to revoke token {}: {}", token_name, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to revoke token: {}", e),
            )))
        }
    }
}

/// Get tokens from database
pub async fn get_tokens_from_db(
    token_service: web::Data<Arc<TokenService>>,
) -> ActixResult<HttpResponse> {
    info!("Getting tokens from database");

    match token_service.get_active_tokens().await {
        Ok(tokens) => {
            info!("Successfully retrieved {} tokens from database", tokens.len());
            Ok(HttpResponse::Ok().json(ApiResponse::success(tokens)))
        }
        Err(e) => {
            error!("Failed to get tokens from database: {}", e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to get tokens: {}", e),
            )))
        }
    }
}

/// Get tokens by project key from database
pub async fn get_tokens_by_project(
    path: web::Path<String>,
    token_service: web::Data<Arc<TokenService>>,
) -> ActixResult<HttpResponse> {
    let project_key = path.into_inner();
    info!("Getting tokens for project: {}", project_key);

    match token_service.get_tokens_by_project(&project_key).await {
        Ok(tokens) => {
            info!("Successfully retrieved {} tokens for project {}", tokens.len(), project_key);
            Ok(HttpResponse::Ok().json(ApiResponse::success(tokens)))
        }
        Err(e) => {
            error!("Failed to get tokens for project {}: {}", project_key, e);
            Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                format!("Failed to get tokens: {}", e),
            )))
        }
    }
}
