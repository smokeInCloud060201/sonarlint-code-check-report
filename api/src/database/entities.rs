use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "api_projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    
    #[sea_orm(unique)]
    pub sonarqube_key: String,
    
    pub name: String,
    
    pub visibility: String,
    
    pub qualifier: String,
    
    pub created_at: DateTimeWithTimeZone,
    
    pub updated_at: DateTimeWithTimeZone,
    
    pub sonarqube_created_at: Option<DateTimeWithTimeZone>,
    
    pub description: Option<String>,
    
    pub language: Option<String>,
    
    pub tags: Option<String>, // JSON string for tags array

    pub project_folder_path: Option<String>, 
    
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            is_active: Set(true),
            ..Default::default()
        }
    }
}

impl Model {
    pub fn from_sonarqube_project(
        sonar_project: &crate::sonarqube::models::ProjectInfo,
        sonarqube_created_at: Option<DateTimeWithTimeZone>,
        project_folder_path: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sonarqube_key: sonar_project.key.clone(),
            name: sonar_project.name.clone(),
            visibility: sonar_project.visibility.clone(),
            qualifier: sonar_project.qualifier.clone(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            sonarqube_created_at,
            description: None,
            language: None,
            tags: None,
            project_folder_path,
            is_active: true,
        }
    }
}

// Re-export as projects module
pub mod projects {
    pub use super::{Entity, Model, ActiveModel, Column};
}
