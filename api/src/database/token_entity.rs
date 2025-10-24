use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sonarqube_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    
    pub name: String,
    
    pub token: String,
    
    pub project_key: Option<String>,
    
    pub created_at: DateTime<Utc>,
    
    pub updated_at: DateTime<Utc>,
    
    pub expires_at: Option<DateTime<Utc>>,
    
    pub is_active: bool,
    
    pub created_by: Option<String>,
    
    pub description: Option<String>,
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
    pub fn from_sonarqube_token(
        name: String,
        token: String,
        project_key: Option<String>,
        created_by: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            token,
            project_key,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: None,
            is_active: true,
            created_by,
            description,
        }
    }
}

// Re-export as tokens module
pub mod tokens {
    pub use super::{Entity, Model, ActiveModel, Column};
}
