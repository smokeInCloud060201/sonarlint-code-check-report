use anyhow::{Result, anyhow};
use tracing::{info, warn, error};

use crate::sonarqube::{
    client::SonarQubeClient,
    models::{
        CreateProjectRequest, CreateProjectResponse, GetIssuesResponse,
        SonarQubeConfig, Issue, CreateTokenRequest, CreateTokenResponse,
        ListTokensResponse,
    },
};
use crate::database::ProjectService;

pub struct SonarQubeService {
    client: SonarQubeClient,
    project_service: ProjectService,
}

impl SonarQubeService {
    pub fn new(config: SonarQubeConfig, project_service: ProjectService) -> Result<Self> {
        let client = SonarQubeClient::new(config)?;
        Ok(Self { client, project_service })
    }

    /// Create a new project in SonarQube
    pub async fn create_project(
        &self,
        name: String,
        project_key: String,
        visibility: Option<String>,
        project_folder_path: Option<String>,
    ) -> Result<CreateProjectResponse> {
        info!("Creating SonarQube project: {} with key: {}", name, project_key);

        let request = CreateProjectRequest {
            name,
            project: project_key,
            visibility,
        };

        // SonarQube uses query parameters for project creation
        let mut endpoint = format!("projects/create?name={}&project={}", 
            urlencoding::encode(&request.name), 
            urlencoding::encode(&request.project)
        );

        if let Some(vis) = &request.visibility {
            endpoint.push_str(&format!("&visibility={}", urlencoding::encode(vis)));
        }

        let response: CreateProjectResponse = self.client.post(&endpoint, &()).await?;
        
        // Save project to database with folder path
        match self.project_service.create_project_with_folder_path(&response.project, project_folder_path).await {
            Ok(db_project) => {
                info!("Successfully saved project to database: {} with folder path: {:?}", 
                      db_project.sonarqube_key, db_project.project_folder_path);
            }
            Err(e) => {
                warn!("Failed to save project to database: {}", e);
                // Continue execution even if database save fails
            }
        }
        
        info!("Successfully created project: {}", response.project.key);
        Ok(response)
    }

    /// Get all issues for a project
    pub async fn get_project_issues(
        &self,
        project_key: String,
        options: Option<IssueQueryOptions>,
    ) -> Result<GetIssuesResponse> {
        info!("Fetching issues for project: {}", project_key);

        let mut endpoint = format!("issues/search?componentKeys={}", 
            urlencoding::encode(&project_key)
        );

        if let Some(opts) = options {
            if let Some(severities) = opts.severities {
                endpoint.push_str(&format!("&severities={}", urlencoding::encode(&severities)));
            }
            if let Some(types) = opts.types {
                endpoint.push_str(&format!("&types={}", urlencoding::encode(&types)));
            }
            if let Some(statuses) = opts.statuses {
                endpoint.push_str(&format!("&statuses={}", urlencoding::encode(&statuses)));
            }
            if let Some(created_after) = opts.created_after {
                endpoint.push_str(&format!("&createdAfter={}", urlencoding::encode(&created_after)));
            }
            if let Some(created_before) = opts.created_before {
                endpoint.push_str(&format!("&createdBefore={}", urlencoding::encode(&created_before)));
            }
            if let Some(page) = opts.page {
                endpoint.push_str(&format!("&p={}", page));
            }
            if let Some(page_size) = opts.page_size {
                endpoint.push_str(&format!("&ps={}", page_size));
            }
        }

        let response: GetIssuesResponse = self.client.get(&endpoint).await?;
        
        info!("Retrieved {} issues for project: {}", response.paging.total, project_key);
        Ok(response)
    }

    /// Get all issues for a project with pagination
    pub async fn get_all_project_issues(
        &self,
        project_key: String,
        options: Option<IssueQueryOptions>,
    ) -> Result<Vec<Issue>> {
        let mut all_issues = Vec::new();
        let mut page = 1;
        let page_size = options.as_ref().and_then(|o| o.page_size).unwrap_or(500);

        loop {
            let mut current_options = options.clone().unwrap_or_default();
            current_options.page = Some(page);
            current_options.page_size = Some(page_size);

            let response = self.get_project_issues(project_key.clone(), Some(current_options)).await?;
            
            let issues_count = response.issues.len();
            all_issues.extend(response.issues);

            // Check if we've reached the end based on paging info
            if issues_count == 0 || page >= (response.paging.total / response.paging.page_size) + 1 {
                break;
            }

            page += 1;
        }

        info!("Retrieved total {} issues for project: {}", all_issues.len(), project_key);
        Ok(all_issues)
    }

    /// Check if a project exists
    pub async fn project_exists(&self, project_key: String) -> Result<bool> {
        let endpoint = format!("projects/search?projects={}", urlencoding::encode(&project_key));
        
        match self.client.get::<serde_json::Value>(&endpoint).await {
            Ok(response) => {
                if let Some(components) = response.get("components").and_then(|c| c.as_array()) {
                    Ok(!components.is_empty())
                } else {
                    Ok(false)
                }
            }
            Err(e) => {
                warn!("Error checking if project exists: {}", e);
                Ok(false)
            }
        }
    }

    /// Delete a project
    pub async fn delete_project(&self, project_key: String) -> Result<()> {
        info!("Deleting SonarQube project: {}", project_key);

        let endpoint = format!("projects/delete?project={}", urlencoding::encode(&project_key));
        
        self.client.get::<serde_json::Value>(&endpoint).await?;
        
        info!("Successfully deleted project: {}", project_key);
        Ok(())
    }

    /// Get project information
    pub async fn get_project_info(&self, project_key: String) -> Result<serde_json::Value> {
        let endpoint = format!("projects/search?projects={}", urlencoding::encode(&project_key));
        
        let response: serde_json::Value = self.client.get(&endpoint).await?;
        Ok(response)
    }

    /// Get SonarQube server status
    pub async fn get_server_version(&self) -> Result<String> {
        let response: serde_json::Value = self.client.get("system/status").await?;
        
        if let Some(version) = response.get("version").and_then(|v| v.as_str()) {
            Ok(version.to_string())
        } else {
            Err(anyhow!("Failed to get SonarQube version"))
        }
    }

    /// Check if SonarQube server is accessible
    pub async fn health_check(&self) -> Result<bool> {
        match self.get_server_version().await {
            Ok(version) => {
                info!("SonarQube server is accessible, version: {}", version);
                Ok(true)
            }
            Err(e) => {
                error!("SonarQube server health check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Generate a new SonarQube token
    pub async fn generate_token(
        &self,
        name: String,
        project_key: Option<String>,
        description: Option<String>,
        expires_at: Option<String>,
    ) -> Result<CreateTokenResponse> {
        info!("Generating SonarQube token: {}", name);

        let request = CreateTokenRequest {
            name: name.clone(),
            project_key: project_key.clone(),
            description: description.clone(),
            expires_at,
        };

        // SonarQube uses query parameters for token generation
        let mut endpoint = format!("user_tokens/generate?name={}", urlencoding::encode(&request.name));

        if let Some(proj_key) = &request.project_key {
            endpoint.push_str(&format!("&projectKey={}", urlencoding::encode(proj_key)));
        }

        if let Some(desc) = &request.description {
            endpoint.push_str(&format!("&description={}", urlencoding::encode(desc)));
        }

        if let Some(expires) = &request.expires_at {
            endpoint.push_str(&format!("&expiresAt={}", urlencoding::encode(expires)));
        }

        let response: CreateTokenResponse = self.client.post(&endpoint, &()).await?;
        
        info!("Successfully generated token: {}", response.name);
        Ok(response)
    }

    /// List all SonarQube tokens
    pub async fn list_tokens(&self) -> Result<ListTokensResponse> {
        info!("Listing SonarQube tokens");

        let response: ListTokensResponse = self.client.get("user_tokens/search").await?;
        
        info!("Retrieved {} tokens", response.tokens.len());
        Ok(response)
    }

    /// Revoke a SonarQube token
    pub async fn revoke_token(&self, name: String) -> Result<()> {
        info!("Revoking SonarQube token: {}", name);

        let endpoint = format!("user_tokens/revoke?name={}", urlencoding::encode(&name));
        
        self.client.post::<(), serde_json::Value>(&endpoint, &()).await?;
        
        info!("Successfully revoked token: {}", name);
        Ok(())
    }
}

/// Options for querying issues
#[derive(Debug, Clone)]
pub struct IssueQueryOptions {
    pub severities: Option<String>,    // BLOCKER, CRITICAL, MAJOR, MINOR, INFO
    pub types: Option<String>,         // CODE_SMELL, BUG, VULNERABILITY, SECURITY_HOTSPOT
    pub statuses: Option<String>,      // OPEN, CONFIRMED, REOPENED, RESOLVED, CLOSED
    pub created_after: Option<String>, // ISO 8601 date format
    pub created_before: Option<String>, // ISO 8601 date format
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl Default for IssueQueryOptions {
    fn default() -> Self {
        Self {
            severities: None,
            types: None,
            statuses: None,
            created_after: None,
            created_before: None,
            page: Some(1),
            page_size: Some(500),
        }
    }
}
