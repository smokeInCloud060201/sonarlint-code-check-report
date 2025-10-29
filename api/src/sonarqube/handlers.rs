use actix_web::{web, HttpResponse, Result};
use crate::database::service::{CreateProjectRequest, ScanProjectRequest, ProjectService, CreateAdminTokenRequest};
use crate::sonarqube::client::SonarQubeClient;
use std::env;

pub async fn create_project(
    req: web::Json<CreateProjectRequest>,
    project_service: web::Data<ProjectService>,
) -> Result<HttpResponse> {
    let sonar_host_url = env::var("SONAR_HOST_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());
    
    // Get USER_TOKEN for admin operations (create/delete projects)
    let admin_token = match project_service.get_admin_token_by_type(&sonar_host_url, "USER_TOKEN").await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "No USER_TOKEN found for this SonarQube instance. Please create a USER_TOKEN first.",
                "suggestion": "Use POST /api/admin-token with token_type: 'USER_TOKEN' (must be created with a user that has admin privileges)"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    };
    
    let sonar_client = SonarQubeClient::new(sonar_host_url.clone(), admin_token);

    // Create project in SonarQube
    if let Err(e) = sonar_client.create_project(&req.project_key, &req.project_name).await {
        return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to create project in SonarQube: {}", e)
        })));
    }

    // Create project in our database
    let mut project_response = match project_service.create_project(req.into_inner()).await {
        Ok(project) => project,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to create project in database: {}", e)
            })));
        }
    };

    // Create token for the project
    match sonar_client.create_project_token(&project_response.project_key).await {
        Ok(token) => {
            // Update project with token
            if let Err(e) = project_service.update_sonar_token(project_response.id, token.clone()).await {
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Failed to update project with token: {}", e)
                })));
            }
            project_response.sonar_token = token;
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to create project token: {}", e)
            })));
        }
    }

    Ok(HttpResponse::Ok().json(project_response))
}

pub async fn get_all_projects(
    project_service: web::Data<ProjectService>,
) -> Result<HttpResponse> {
    match project_service.get_all_projects().await {
        Ok(projects) => Ok(HttpResponse::Ok().json(projects)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })))
        }
    }
}

pub async fn create_admin_token(
    req: web::Json<CreateAdminTokenRequest>,
    project_service: web::Data<ProjectService>,
) -> Result<HttpResponse> {
    let sonar_client = SonarQubeClient::new(req.sonar_host_url.clone(), String::new());

    // Validate token_type
    let token_type = if req.token_type == "GLOBAL_ANALYSIS_TOKEN" {
        "GLOBAL_ANALYSIS_TOKEN"
    } else {
        "USER_TOKEN"
    };

    // Generate admin token in SonarQube with the specified type
    let token_value = match sonar_client.generate_admin_token(&req.username, &req.password, &req.token_name, token_type).await {
        Ok(token) => token,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to generate admin token in SonarQube: {}", e)
            })));
        }
    };

    // Create admin token request with validated token_type
    let mut create_request = req.into_inner();
    create_request.token_type = token_type.to_string();

    // Create admin token in our database
    let mut admin_token_response = match project_service.create_admin_token(create_request).await {
        Ok(admin_token) => admin_token,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to create admin token in database: {}", e)
            })));
        }
    };

    // Update admin token with the generated value
    if let Err(e) = project_service.update_admin_token_value(admin_token_response.id, token_value.clone()).await {
        return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to update admin token with value: {}", e)
        })));
    }

    admin_token_response.token_value = token_value;
    Ok(HttpResponse::Ok().json(admin_token_response))
}

pub async fn get_project_results(
    req: web::Json<ScanProjectRequest>,
    project_service: web::Data<ProjectService>,
) -> Result<HttpResponse> {
    // Find project by path
    let project = match project_service.get_project_by_path(&req.project_path).await {
        Ok(Some(project)) => project,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Project not found"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    };

    // Get results from SonarQube
    let sonar_host_url = env::var("SONAR_HOST_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());
    
    // Get GLOBAL_ANALYSIS_TOKEN for fetching issues, coverage, etc.
    let admin_token = match project_service.get_admin_token_by_type(&sonar_host_url, "GLOBAL_ANALYSIS_TOKEN").await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "No GLOBAL_ANALYSIS_TOKEN found for this SonarQube instance. Please create a GLOBAL_ANALYSIS_TOKEN first.",
                "suggestion": "Use POST /api/admin-token with token_type: 'GLOBAL_ANALYSIS_TOKEN'"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    };
    
    let sonar_client = SonarQubeClient::new(sonar_host_url, admin_token);
    
    // Fetch issues, coverage, and quality gate in parallel
    let issues_result = sonar_client.get_project_issues(&project.project_key).await;
    let coverage_result = sonar_client.get_project_coverage(&project.project_key).await;
    let quality_gate_result = sonar_client.get_project_quality_gate(&project.project_key).await;

    let mut response_data = serde_json::json!({
        "project": project,
    });

    // Handle issues response
    match issues_result {
        Ok(issues_response) => {
            response_data["issues"] = serde_json::to_value(&issues_response).unwrap_or(serde_json::Value::Null);
        }
        Err(e) => {
            println!("Error fetching issues: {}", e);
            response_data["issues_error"] = serde_json::json!({
                "error": format!("Failed to fetch issues: {}", e)
            });
        }
    }

    // Handle coverage response
    match coverage_result {
        Ok(coverage_response) => {
            response_data["coverage"] = serde_json::to_value(&coverage_response).unwrap_or(serde_json::Value::Null);
        }
        Err(e) => {
            println!("Error fetching coverage: {}", e);
            // Check if it's a decoding error (likely no coverage data available)
            if e.to_string().contains("decoding response body") {
                response_data["coverage"] = serde_json::json!({
                    "message": "No coverage data available for this project. Please ensure the project has been analyzed with coverage reports.",
                    "component": {
                        "measures": []
                    }
                });
            } else {
                response_data["coverage_error"] = serde_json::json!({
                    "error": format!("Failed to fetch coverage: {}", e)
                });
            }
        }
    }

    // Handle quality gate response
    match quality_gate_result {
        Ok(quality_gate_response) => {
            response_data["quality_gate"] = serde_json::to_value(&quality_gate_response).unwrap_or(serde_json::Value::Null);
        }
        Err(e) => {
            println!("Error fetching quality gate: {}", e);
            // Check if it's a decoding error (likely no quality gate data available)
            if e.to_string().contains("decoding response body") {
                response_data["quality_gate"] = serde_json::json!({
                    "message": "No quality gate data available for this project. Please ensure the project has been analyzed and quality gate is configured.",
                    "projectStatus": {
                        "status": "UNKNOWN",
                        "conditions": []
                    }
                });
            } else {
                response_data["quality_gate_error"] = serde_json::json!({
                    "error": format!("Failed to fetch quality gate: {}", e)
                });
            }
        }
    }

    Ok(HttpResponse::Ok().json(response_data))
}

pub async fn generate_sonar_command(
    req: web::Json<ScanProjectRequest>,
    project_service: web::Data<ProjectService>,
) -> Result<HttpResponse> {
    // Find project by path
    let project = match project_service.get_project_by_path(&req.project_path).await {
        Ok(Some(project)) => project,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Project not found"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    };

    // Generate the sonar command
    let mut command = format!(
        "./gradlew test sonar -Dsonar.token={} -Dsonar.host.url={} -Dsonar.projectKey={} -Dsonar.projectName={}",
        project.sonar_token,
        project.sonar_host_url,
        project.project_key,
        project.project_name
    );

    // Add coverage report path if available
    if let Some(coverage_path) = &project.coverage_report_path {
        command = format!("{} -Dsonar.coverage.jacoco.xmlReportPaths={}", command, coverage_path);
    }

    // Add language
    command = format!("{} -Dsonar.language={}", command, project.language);

    // Add sources and tests paths
    command = format!("{} -Dsonar.sources={}", command, project.sources_path);
    command = format!("{} -Dsonar.tests={}", command, project.tests_path);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "command": command,
        "project_path": project.project_path
    })))
}

pub async fn delete_project(
    req: web::Json<ScanProjectRequest>,
    project_service: web::Data<ProjectService>,
) -> Result<HttpResponse> {
    let sonar_host_url = env::var("SONAR_HOST_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());
    
    // Find project by path
    let project = match project_service.get_project_by_path(&req.project_path).await {
        Ok(Some(project)) => project,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Project not found"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    };

    // Get USER_TOKEN for admin operations (create/delete projects)
    let admin_token = match project_service.get_admin_token_by_type(&sonar_host_url, "USER_TOKEN").await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "No USER_TOKEN found for this SonarQube instance. Please create a USER_TOKEN first.",
                "suggestion": "Use POST /api/admin-token with token_type: 'USER_TOKEN' (must be created with a user that has admin privileges)"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    };
    
    let sonar_client = SonarQubeClient::new(sonar_host_url.clone(), admin_token);
    
    // Delete project from SonarQube first
    let sonar_delete_result = sonar_client.delete_project(&project.project_key).await;
    let mut sonar_delete_error = None;
    
    if let Err(e) = sonar_delete_result {
        let error_msg = e.to_string();
        sonar_delete_error = Some(error_msg.clone());
        
        // Check if it's a privileges error (case-insensitive, check multiple variations)
        let error_lower = error_msg.to_lowercase();
        if error_lower.contains("insufficient privileges") 
            || error_lower.contains("privilege") 
            || error_lower.contains("not authorized")
            || error_lower.contains("unauthorized")
            || error_lower.contains("access denied") {
            // Return a helpful error message - don't delete from database
            return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Insufficient privileges to delete project from SonarQube",
                "details": "The admin token does not have the necessary permissions to delete projects. Please ensure the token has admin privileges in SonarQube, or create a new admin token with proper permissions.",
                "sonar_error": error_msg,
                "suggestion": "You may need to recreate the admin token with a user that has administrator permissions, or manually delete the project from SonarQube UI.",
                "project_key": project.project_key,
                "project_path": project.project_path,
                "note": "Project was NOT deleted from database due to insufficient privileges. Please fix permissions and try again, or delete manually from both SonarQube and database."
            })));
        }
        
        // For other errors, we'll continue with database deletion but warn the user
        println!("Warning: Failed to delete project from SonarQube: {}", error_msg);
    }

    // Delete project from database
    match project_service.delete_project_by_path(&req.project_path).await {
        Ok(Some(_)) => {
            // If SonarQube deletion failed with non-privilege error, include warning
            if let Some(error) = sonar_delete_error {
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "message": "Project deleted from database successfully",
                    "warning": "Failed to delete project from SonarQube",
                    "sonar_error": error,
                    "project_key": project.project_key,
                    "project_path": project.project_path,
                    "note": "Project has been removed from local database. You may need to manually delete it from SonarQube."
                })))
            } else {
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "message": "Project deleted successfully from both SonarQube and database",
                    "project_key": project.project_key,
                    "project_path": project.project_path
                })))
            }
        }
        Ok(None) => {
            // Project was already deleted or doesn't exist
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Project not found in database"
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to delete project from database: {}", e)
            })))
        }
    }
}
