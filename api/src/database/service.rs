use std::env;
use crate::database::entities::Entity as ProjectEntity;
use crate::database::entities::Model as ProjectModel;
use crate::database::entities::ActiveModel as ProjectActiveModel;
use crate::database::admin_token_entity::Entity as AdminTokenEntity;
use crate::database::admin_token_entity::ActiveModel as AdminTokenActiveModel;
use sea_orm::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub project_key: String,
    pub project_name: String,
    pub project_path: String,
    pub language: String,
    pub sources_path: String,
    pub tests_path: String,
    pub coverage_report_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanProjectRequest {
    pub project_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAdminTokenRequest {
    pub username: String,
    pub password: String,
    pub token_name: String,
    pub token_type: String, // "USER_TOKEN" or "GLOBAL_ANALYSIS_TOKEN"
    pub sonar_host_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminTokenResponse {
    pub id: i32,
    pub username: String,
    pub token_name: String,
    pub token_value: String,
    pub token_type: String,
    pub sonar_host_url: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectResponse {
    pub id: i32,
    pub project_key: String,
    pub project_name: String,
    pub project_path: String,
    pub sonar_token: String,
    pub sonar_host_url: String,
    pub language: String,
    pub sources_path: String,
    pub tests_path: String,
    pub coverage_report_path: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<ProjectModel> for ProjectResponse {
    fn from(model: ProjectModel) -> Self {
        Self {
            id: model.id,
            project_key: model.project_key,
            project_name: model.project_name,
            project_path: model.project_path,
            sonar_token: model.sonar_token,
            sonar_host_url: model.sonar_host_url,
            language: model.language,
            sources_path: model.sources_path,
            tests_path: model.tests_path,
            coverage_report_path: model.coverage_report_path,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Clone)]
pub struct ProjectService {
    db: DatabaseConnection,
}

impl ProjectService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_project(&self, request: CreateProjectRequest) -> Result<ProjectResponse, DbErr> {
        let now = Utc::now().naive_utc();
        let sonar_host_url = env::var("SONAR_HOST_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());

        let project = ProjectActiveModel {
            project_key: Set(request.project_key),
            project_name: Set(request.project_name),
            project_path: Set(request.project_path),
            sonar_token: Set(String::new()), // Will be set after SonarQube token creation
            sonar_host_url: Set(sonar_host_url), // Default SonarQube URL
            language: Set(request.language),
            sources_path: Set(request.sources_path),
            tests_path: Set(request.tests_path),
            coverage_report_path: Set(request.coverage_report_path),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let result = project.insert(&self.db).await?;
        Ok(ProjectResponse::from(result))
    }

    pub async fn get_project_by_path(&self, project_path: &str) -> Result<Option<ProjectResponse>, DbErr> {
        let project = ProjectEntity::find()
            .filter(crate::database::entities::Column::ProjectPath.eq(project_path))
            .one(&self.db)
            .await?;

        Ok(project.map(ProjectResponse::from))
    }

    pub async fn update_sonar_token(&self, project_id: i32, token: String) -> Result<(), DbErr> {
        let project = ProjectEntity::find_by_id(project_id).one(&self.db).await?;
        
        if let Some(project) = project {
            let mut project: ProjectActiveModel = project.into();
            project.sonar_token = Set(token);
            project.updated_at = Set(Utc::now().naive_utc());
            project.update(&self.db).await?;
        }

        Ok(())
    }

    pub async fn get_all_projects(&self) -> Result<Vec<ProjectResponse>, DbErr> {
        let projects = ProjectEntity::find().all(&self.db).await?;
        Ok(projects.into_iter().map(ProjectResponse::from).collect())
    }

    pub async fn create_admin_token(&self, request: CreateAdminTokenRequest) -> Result<AdminTokenResponse, DbErr> {
        let now = Utc::now().naive_utc();
        
        // Validate token_type
        let token_type = if request.token_type == "GLOBAL_ANALYSIS_TOKEN" || request.token_type == "USER_TOKEN" {
            request.token_type
        } else {
            "USER_TOKEN".to_string() // Default to USER_TOKEN if invalid
        };
        
        let admin_token = AdminTokenActiveModel {
            username: Set(request.username),
            token_name: Set(request.token_name),
            token_value: Set(String::new()), // Will be set after SonarQube token creation
            token_type: Set(token_type),
            sonar_host_url: Set(request.sonar_host_url),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let result = admin_token.insert(&self.db).await?;
        Ok(AdminTokenResponse {
            id: result.id,
            username: result.username,
            token_name: result.token_name,
            token_value: result.token_value,
            token_type: result.token_type,
            sonar_host_url: result.sonar_host_url,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }

    pub async fn get_admin_token_by_type(&self, sonar_host_url: &str, token_type: &str) -> Result<Option<String>, DbErr> {
        let admin_token = AdminTokenEntity::find()
            .filter(crate::database::admin_token_entity::Column::SonarHostUrl.eq(sonar_host_url))
            .filter(crate::database::admin_token_entity::Column::TokenType.eq(token_type))
            .one(&self.db)
            .await?;

        Ok(admin_token.map(|token| token.token_value))
    }

    pub async fn update_admin_token_value(&self, token_id: i32, token_value: String) -> Result<(), DbErr> {
        let admin_token = AdminTokenEntity::find_by_id(token_id).one(&self.db).await?;
        
        if let Some(admin_token) = admin_token {
            let mut admin_token: AdminTokenActiveModel = admin_token.into();
            admin_token.token_value = Set(token_value);
            admin_token.updated_at = Set(Utc::now().naive_utc());
            admin_token.update(&self.db).await?;
        }

        Ok(())
    }

    pub async fn delete_project_by_path(&self, project_path: &str) -> Result<Option<ProjectResponse>, DbErr> {
        let project = ProjectEntity::find()
            .filter(crate::database::entities::Column::ProjectPath.eq(project_path))
            .one(&self.db)
            .await?;

        if let Some(project) = project {
            let project_response = ProjectResponse::from(project.clone());
            let project_id = project.id;
            ProjectEntity::delete_by_id(project_id).exec(&self.db).await?;
            Ok(Some(project_response))
        } else {
            Ok(None)
        }
    }
}
