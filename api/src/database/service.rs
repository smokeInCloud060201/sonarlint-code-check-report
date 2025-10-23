use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait, QueryOrder};
use anyhow::{Result, anyhow};
use tracing::{info, error};
use uuid::Uuid;
use chrono::Utc;

use crate::database::entities::projects::{Entity as ProjectEntity, Model as ProjectModel, ActiveModel as ProjectActiveModel, Column};
use crate::sonarqube::models::ProjectInfo;

#[derive(Clone)]
pub struct ProjectService {
    db: DatabaseConnection,
}

impl ProjectService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Create a new project in the database
    pub async fn create_project(&self, sonar_project: &ProjectInfo) -> Result<ProjectModel> {
        info!("Creating project in database: {}", sonar_project.key);

        // Check if project already exists
        if let Some(existing) = self.find_by_sonarqube_key(&sonar_project.key).await? {
            info!("Project already exists in database: {}", sonar_project.key);
            return Ok(existing);
        }

        let project_model = ProjectModel::from_sonarqube_project(sonar_project, None);
        
        let active_model = ProjectActiveModel {
            id: Set(project_model.id),
            sonarqube_key: Set(project_model.sonarqube_key),
            name: Set(project_model.name),
            visibility: Set(project_model.visibility),
            qualifier: Set(project_model.qualifier),
            created_at: Set(project_model.created_at),
            updated_at: Set(project_model.updated_at),
            sonarqube_created_at: Set(project_model.sonarqube_created_at),
            description: Set(project_model.description),
            language: Set(project_model.language),
            tags: Set(project_model.tags),
            is_active: Set(project_model.is_active),
        };

        match active_model.insert(&self.db).await {
            Ok(inserted_project) => {
                info!("Successfully created project in database: {}", inserted_project.sonarqube_key);
                Ok(inserted_project)
            }
            Err(e) => {
                error!("Failed to create project in database: {}", e);
                Err(anyhow!("Database error: {}", e))
            }
        }
    }

    /// Find project by SonarQube key
    pub async fn find_by_sonarqube_key(&self, sonarqube_key: &str) -> Result<Option<ProjectModel>> {
        match ProjectEntity::find()
            .filter(Column::SonarqubeKey.eq(sonarqube_key))
            .one(&self.db)
            .await
        {
            Ok(project) => Ok(project),
            Err(e) => {
                error!("Failed to find project by SonarQube key {}: {}", sonarqube_key, e);
                Err(anyhow!("Database error: {}", e))
            }
        }
    }

    /// Find project by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<ProjectModel>> {
        match ProjectEntity::find_by_id(id).one(&self.db).await {
            Ok(project) => Ok(project),
            Err(e) => {
                error!("Failed to find project by ID {}: {}", id, e);
                Err(anyhow!("Database error: {}", e))
            }
        }
    }

    /// Get all active projects
    pub async fn get_all_active_projects(&self) -> Result<Vec<ProjectModel>> {
        match ProjectEntity::find()
            .filter(Column::IsActive.eq(true))
            .order_by_desc(Column::CreatedAt)
            .all(&self.db)
            .await
        {
            Ok(projects) => {
                info!("Retrieved {} active projects from database", projects.len());
                Ok(projects)
            }
            Err(e) => {
                error!("Failed to get active projects: {}", e);
                Err(anyhow!("Database error: {}", e))
            }
        }
    }

    /// Update project information
    pub async fn update_project(&self, id: Uuid, updates: ProjectUpdateData) -> Result<ProjectModel> {
        let mut project: ProjectActiveModel = match self.find_by_id(id).await? {
            Some(p) => p.into(),
            None => return Err(anyhow!("Project not found")),
        };

        if let Some(name) = updates.name {
            project.name = Set(name);
        }
        if let Some(visibility) = updates.visibility {
            project.visibility = Set(visibility);
        }
        if let Some(description) = updates.description {
            project.description = Set(Some(description));
        }
        if let Some(language) = updates.language {
            project.language = Set(Some(language));
        }
        if let Some(tags) = updates.tags {
            project.tags = Set(Some(tags));
        }

        project.updated_at = Set(Utc::now());

        match project.update(&self.db).await {
            Ok(updated_project) => {
                info!("Successfully updated project: {}", updated_project.sonarqube_key);
                Ok(updated_project)
            }
            Err(e) => {
                error!("Failed to update project: {}", e);
                Err(anyhow!("Database error: {}", e))
            }
        }
    }

    /// Soft delete project (mark as inactive)
    pub async fn deactivate_project(&self, id: Uuid) -> Result<ProjectModel> {
        let mut project: ProjectActiveModel = match self.find_by_id(id).await? {
            Some(p) => p.into(),
            None => return Err(anyhow!("Project not found")),
        };

        project.is_active = Set(false);
        project.updated_at = Set(Utc::now());

        match project.update(&self.db).await {
            Ok(updated_project) => {
                info!("Successfully deactivated project: {}", updated_project.sonarqube_key);
                Ok(updated_project)
            }
            Err(e) => {
                error!("Failed to deactivate project: {}", e);
                Err(anyhow!("Database error: {}", e))
            }
        }
    }

    /// Hard delete project from database
    pub async fn delete_project(&self, id: Uuid) -> Result<()> {
        match ProjectEntity::delete_by_id(id).exec(&self.db).await {
            Ok(_) => {
                info!("Successfully deleted project with ID: {}", id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to delete project: {}", e);
                Err(anyhow!("Database error: {}", e))
            }
        }
    }
}

#[derive(Debug)]
pub struct ProjectUpdateData {
    pub name: Option<String>,
    pub visibility: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub tags: Option<String>,
}
