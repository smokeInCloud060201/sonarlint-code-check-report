use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    
    #[sea_orm(unique)]
    pub sonarqube_key: String,
    
    pub name: String,
    
    pub visibility: String,
    
    pub qualifier: String,
    
    pub created_at: DateTime<Utc>,
    
    pub updated_at: DateTime<Utc>,
    
    pub sonarqube_created_at: Option<DateTime<Utc>>,
    
    pub description: Option<String>,
    
    pub language: Option<String>,
    
    pub tags: Option<String>, // JSON string for tags array
    
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            is_active: Set(true),
            ..Default::default()
        }
    }
}

impl Model {
    pub fn from_sonarqube_project(
        sonar_project: &crate::sonarqube::models::ProjectInfo,
        sonarqube_created_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sonarqube_key: sonar_project.key.clone(),
            name: sonar_project.name.clone(),
            visibility: sonar_project.visibility.clone(),
            qualifier: sonar_project.qualifier.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            sonarqube_created_at,
            description: None,
            language: None,
            tags: None,
            is_active: true,
        }
    }
}

// Re-export as projects module
pub mod projects {
    pub use super::{Entity, Model, ActiveModel, Column};
}
