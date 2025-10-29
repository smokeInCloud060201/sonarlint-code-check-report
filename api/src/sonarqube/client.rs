use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIssuesResponse {
    pub issues: Vec<Issue>,
    pub paging: Paging,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub key: String,
    pub rule: String,
    pub severity: String,
    pub component: String,
    pub project: String,
    pub line: Option<i32>,
    pub message: String,
    pub effort: Option<String>,
    pub debt: Option<String>,
    pub status: String,
    pub resolution: Option<String>,
    pub hash: Option<String>,
    pub author: Option<String>,
    #[serde(rename = "creationDate")]
    pub creation_date: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub issue_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paging {
    #[serde(rename = "pageIndex")]
    pub page_index: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoverageResponse {
    pub component: Component,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityGateResponse {
    #[serde(rename = "projectStatus")]
    pub project_status: ProjectStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStatus {
    pub status: String, // "OK", "WARN", "ERROR"
    pub conditions: Vec<Condition>,
    #[serde(rename = "ignoredConditions")]
    pub ignored_conditions: Option<bool>,
    pub period: Option<QualityGatePeriod>,
    #[serde(rename = "caycStatus")]
    pub cayc_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityGatePeriod {
    pub mode: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    pub status: String, // "OK", "WARN", "ERROR"
    #[serde(rename = "metricKey")]
    pub metric_key: String,
    pub comparator: String, // "GT", "LT", "EQ"
    #[serde(rename = "errorThreshold")]
    pub error_threshold: Option<String>,
    #[serde(rename = "actualValue")]
    pub actual_value: Option<String>,
    #[serde(rename = "periodIndex")]
    pub period_index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub id: Option<String>,
    pub key: String,
    pub name: String,
    pub qualifier: String,
    pub measures: Vec<Measure>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measure {
    pub metric: String,
    pub value: String,
    #[serde(rename = "bestValue")]
    pub best_value: Option<bool>,
    pub periods: Option<Vec<Period>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Period {
    #[serde(rename = "index")]
    pub index: i32,
    pub value: String,
    pub date: String,
}

pub struct SonarQubeClient {
    client: Client,
    base_url: String,
    admin_token: String,
}

impl SonarQubeClient {
    pub fn new(base_url: String, admin_token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            admin_token,
        }
    }

    pub async fn create_project(&self, project_key: &str, project_name: &str) -> Result<()> {
        let url = format!("{}/api/projects/create", self.base_url);
        
        // SonarQube API expects form-encoded data, not JSON
        let params = [
            ("project", project_key),
            ("name", project_name),
        ];

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:", self.admin_token))))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to create project: {}", error_text));
        }

        Ok(())
    }

    pub async fn create_project_token(&self, project_key: &str) -> Result<String> {
        let url = format!("{}/api/user_tokens/generate", self.base_url);
        
        // SonarQube API expects form-encoded data
        let params = [
            ("name", format!("{}_token", project_key)),
            ("type", "PROJECT_ANALYSIS_TOKEN".to_string()),
            ("projectKey", project_key.to_string()),
        ];

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:", self.admin_token))))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to create token: {}", error_text));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response.token)
    }

    pub async fn get_project_issues(&self, project_key: &str) -> Result<ProjectIssuesResponse> {
        let url = format!("{}/api/issues/search", self.base_url);
        
        let params = [
            ("componentKeys", project_key),
            ("resolved", "false"),
            ("ps", "500"),
        ];

        let response = self.client
            .get(&url)
            .query(&params)
            .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:", self.admin_token))))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to get project issues: {}", error_text));
        }

        let issues_response: ProjectIssuesResponse = response.json().await?;
        Ok(issues_response)
    }

    pub async fn get_project_coverage(&self, project_key: &str) -> Result<CoverageResponse> {
        let url = format!("{}/api/measures/component", self.base_url);
        
        let params = [
            ("component", project_key),
            ("metricKeys", "coverage,branch_coverage,line_coverage,lines_to_cover,uncovered_lines"),
        ];

        let response = self.client
            .get(&url)
            .query(&params)
            .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:", self.admin_token))))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to get project coverage: {}", error_text));
        }

        let coverage_response: CoverageResponse = response.json().await?;
    Ok(coverage_response)
}

pub async fn get_project_quality_gate(&self, project_key: &str) -> Result<QualityGateResponse> {
    let url = format!("{}/api/qualitygates/project_status", self.base_url);
    
    let params = [
        ("projectKey", project_key),
    ];

        let response = self.client
            .get(&url)
            .query(&params)
            .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:", self.admin_token))))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to get project quality gate: {}", error_text));
        }

        let quality_gate_response: QualityGateResponse = response.json().await?;
        Ok(quality_gate_response)
}

pub async fn generate_admin_token(&self, username: &str, password: &str, token_name: &str) -> Result<String> {
        let url = format!("{}/api/user_tokens/generate", self.base_url);
        
        // SonarQube API expects form-encoded data
        let params = [
            ("name", token_name.to_string()),
            ("type", "GLOBAL_ANALYSIS_TOKEN".to_string()),
        ];

        let response = self.client
            .post(&url)
            .form(&params)
            .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:{}", username, password))))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to generate admin token: {}", error_text));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response.token)
    }
}
