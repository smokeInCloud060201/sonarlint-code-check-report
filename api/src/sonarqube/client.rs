use reqwest::{Client, ClientBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use anyhow::{Result, anyhow};
use tracing::{error, debug, info};

use crate::sonarqube::models::{SonarQubeConfig, SonarQubeError};

pub struct SonarQubeClient {
    client: Client,
    config: SonarQubeConfig,
}

impl SonarQubeClient {
    pub fn new(config: SonarQubeConfig) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self { client, config })
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/api/{}", self.config.base_url, endpoint);
        debug!("Making GET request to: {}", url);

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post<T, U>(&self, endpoint: &str, body: &T) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = format!("{}/api/{}", self.config.base_url, endpoint);
        debug!("Making POST request to: {}", url);

        info!("&self.config.username, Some(&self.config.password) {} {}", &self.config.username, &self.config.password);

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn put<T, U>(&self, endpoint: &str, body: &T) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = format!("{}/api/{}", self.config.base_url, endpoint);
        debug!("Making PUT request to: {}", url);

        let response = self
            .client
            .put(&url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn delete<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/api/{}", self.config.base_url, endpoint);
        debug!("Making DELETE request to: {}", url);

        let response = self
            .client
            .delete(&url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            error!("SonarQube API error: {} - {}", status, response_text);
            
            // Try to parse SonarQube error format
            if let Ok(sonar_error) = serde_json::from_str::<SonarQubeError>(&response_text) {
                let error_msg = sonar_error.errors
                    .into_iter()
                    .map(|e| e.msg)
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(anyhow!("SonarQube API error: {}", error_msg));
            }
            
            return Err(anyhow!("SonarQube API error: {} - {}", status, response_text));
        }

        serde_json::from_str(&response_text)
            .map_err(|e| anyhow!("Failed to parse SonarQube response: {} - {}", e, response_text))
    }

    pub fn get_config(&self) -> &SonarQubeConfig {
        &self.config
    }
}
