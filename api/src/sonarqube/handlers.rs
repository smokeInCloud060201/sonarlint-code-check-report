use actix_web::{web, HttpResponse, Result};
use crate::database::service::{CreateProjectRequest, ScanProjectRequest, ProjectService, CreateAdminTokenRequest};
use crate::sonarqube::client::SonarQubeClient;
use std::env;

pub async fn create_project(
    req: web::Json<CreateProjectRequest>,
    project_service: web::Data<ProjectService>,
) -> Result<HttpResponse> {
    let sonar_host_url = env::var("SONAR_HOST_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());
    
    // Get admin token from database
    let admin_token = match project_service.get_admin_token(&sonar_host_url).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "No admin token found for this SonarQube instance. Please create an admin token first."
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

    // Generate admin token in SonarQube
    let token_value = match sonar_client.generate_admin_token(&req.username, &req.password, &req.token_name).await {
        Ok(token) => token,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to generate admin token in SonarQube: {}", e)
            })));
        }
    };

    // Create admin token in our database
    let mut admin_token_response = match project_service.create_admin_token(req.into_inner()).await {
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
    
    // Get admin token from database
    let admin_token = match project_service.get_admin_token(&sonar_host_url).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "No admin token found for this SonarQube instance. Please create an admin token first."
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
        "./gradlew sonar -Dsonar.token={} -Dsonar.host.url={} -Dsonar.projectKey={} -Dsonar.projectName={}",
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
