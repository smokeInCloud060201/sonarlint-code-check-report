use anyhow::{Result, anyhow};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait};
use tracing::{info, error, warn};
use uuid::Uuid;
use chrono::Utc;

use crate::database::token_entity::{Entity as TokenEntity, Model as TokenModel, ActiveModel as TokenActiveModel};

pub struct TokenService {
    db: DatabaseConnection,
}

impl TokenService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Create a new token in the database
    pub async fn create_token(
        &self,
        name: String,
        token: String,
        project_key: Option<String>,
        created_by: Option<String>,
        description: Option<String>,
    ) -> Result<TokenModel> {
        info!("Creating token in database: {}", name);

        let token_model = TokenModel::from_sonarqube_token(
            name.clone(),
            token,
            project_key,
            created_by,
            description,
        );

        let active_model = TokenActiveModel {
            id: Set(token_model.id),
            name: Set(token_model.name),
            token: Set(token_model.token),
            project_key: Set(token_model.project_key),
            created_at: Set(token_model.created_at),
            updated_at: Set(token_model.updated_at),
            expires_at: Set(token_model.expires_at),
            is_active: Set(token_model.is_active),
            created_by: Set(token_model.created_by),
            description: Set(token_model.description),
        };

        match active_model.insert(&self.db).await {
            Ok(inserted_token) => {
                info!("Successfully created token in database: {}", inserted_token.name);
                Ok(inserted_token)
            }
            Err(e) => {
                error!("Failed to create token in database: {}", e);
                Err(anyhow!("Failed to create token: {}", e))
            }
        }
    }

    /// Get a token by ID
    pub async fn get_token_by_id(&self, id: Uuid) -> Result<Option<TokenModel>> {
        match TokenEntity::find_by_id(id).one(&self.db).await {
            Ok(token) => Ok(token),
            Err(e) => {
                error!("Failed to get token by ID {}: {}", id, e);
                Err(anyhow!("Failed to get token: {}", e))
            }
        }
    }

    /// Get a token by name
    pub async fn get_token_by_name(&self, name: &str) -> Result<Option<TokenModel>> {
        match TokenEntity::find()
            .filter(crate::database::token_entity::Column::Name.eq(name))
            .one(&self.db)
            .await
        {
            Ok(token) => Ok(token),
            Err(e) => {
                error!("Failed to get token by name {}: {}", name, e);
                Err(anyhow!("Failed to get token: {}", e))
            }
        }
    }

    /// Get all active tokens
    pub async fn get_active_tokens(&self) -> Result<Vec<TokenModel>> {
        match TokenEntity::find()
            .filter(crate::database::token_entity::Column::IsActive.eq(true))
            .all(&self.db)
            .await
        {
            Ok(tokens) => Ok(tokens),
            Err(e) => {
                error!("Failed to get active tokens: {}", e);
                Err(anyhow!("Failed to get tokens: {}", e))
            }
        }
    }

    /// Get tokens by project key
    pub async fn get_tokens_by_project(&self, project_key: &str) -> Result<Vec<TokenModel>> {
        match TokenEntity::find()
            .filter(crate::database::token_entity::Column::ProjectKey.eq(project_key))
            .filter(crate::database::token_entity::Column::IsActive.eq(true))
            .all(&self.db)
            .await
        {
            Ok(tokens) => Ok(tokens),
            Err(e) => {
                error!("Failed to get tokens for project {}: {}", project_key, e);
                Err(anyhow!("Failed to get tokens: {}", e))
            }
        }
    }

    /// Deactivate a token (soft delete)
    pub async fn deactivate_token(&self, id: Uuid) -> Result<()> {
        match TokenEntity::find_by_id(id).one(&self.db).await {
            Ok(Some(token)) => {
                let mut active_model: TokenActiveModel = token.into();
                active_model.is_active = Set(false);
                active_model.updated_at = Set(Utc::now());

                match active_model.update(&self.db).await {
                    Ok(_) => {
                        info!("Successfully deactivated token: {}", id);
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to deactivate token {}: {}", id, e);
                        Err(anyhow!("Failed to deactivate token: {}", e))
                    }
                }
            }
            Ok(None) => {
                warn!("Token not found: {}", id);
                Err(anyhow!("Token not found"))
            }
            Err(e) => {
                error!("Failed to find token {}: {}", id, e);
                Err(anyhow!("Failed to find token: {}", e))
            }
        }
    }

    /// Delete a token permanently
    pub async fn delete_token(&self, id: Uuid) -> Result<()> {
        match TokenEntity::delete_by_id(id).exec(&self.db).await {
            Ok(_) => {
                info!("Successfully deleted token: {}", id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to delete token {}: {}", id, e);
                Err(anyhow!("Failed to delete token: {}", e))
            }
        }
    }
}
