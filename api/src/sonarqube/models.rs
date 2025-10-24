use serde::{Deserialize, Serialize};

// Project Creation Models
#[derive(Debug, Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub project: String,
    pub visibility: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProjectResponse {
    pub project: ProjectInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectInfo {
    pub key: String,
    pub name: String,
    pub qualifier: String,
    pub visibility: String,
}

// Issue Models
#[derive(Debug, Serialize)]
pub struct GetIssuesRequest {
    pub component_keys: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetIssuesResponse {
    pub paging: Paging,
    pub issues: Vec<Issue>,
    pub components: Vec<Component>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paging {
    #[serde(rename="pageIndex")]
    pub page_index: u32,

    #[serde(rename="pageSize")]
    pub page_size: u32,
    pub total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    pub key: String,
    pub component: String,
    pub project: String,
    pub rule: String,
    pub clean_code_attribute: Option<String>,
    pub clean_code_attribute_category: Option<String>,
    pub issue_status: String,
    pub prioritized_rule: bool,
    pub impacts: Vec<Impact>,
    pub message: String,
    pub message_formattings: Vec<MessageFormatting>,
    pub line: Option<u32>,
    pub hash: Option<String>,
    pub author: Option<String>,
    pub effort: Option<String>,
    pub creation_date: String,
    pub update_date: String,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
    pub transitions: Vec<String>,
    pub actions: Vec<String>,
    pub text_range: Option<TextRange>,
    pub flows: Vec<Flow>,
    pub quick_fix_available: Option<bool>,
    pub rule_description_context_key: Option<String>,
    pub code_variants: Vec<String>,
    pub internal_tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Impact {
    pub software_quality: String,
    pub severity: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageFormatting {
    pub start: u32,
    pub end: u32,
    #[serde(rename = "type")]
    pub formatting_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextRange {
    pub start_line: u32,
    pub end_line: u32,
    pub start_offset: u32,
    pub end_offset: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Flow {
    pub locations: Vec<FlowLocation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlowLocation {
    pub text_range: Option<TextRange>,
    pub msg: Option<String>,
    pub msg_formattings: Vec<MessageFormatting>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub key: String,
    pub login: String,
    pub html_text: String,
    pub markdown: String,
    pub updatable: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    pub key: String,
    pub enabled: bool,
    pub qualifier: String,
    pub name: String,
    pub long_name: String,
    pub path: Option<String>,
    pub language: Option<String>,
    pub measures: Vec<Measure>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Measure {
    pub metric: String,
    pub value: Option<String>,
    pub periods: Vec<Period>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Period {
    pub index: u32,
    pub mode: String,
    pub date: String,
    pub parameter: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct DescriptionSection {
    pub key: String,
    pub content: String,
    pub context: Option<Context>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    pub key: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub name: String,
    pub active: bool,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Facet {
    pub property: String,
    pub values: Vec<FacetValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FacetValue {
    pub val: String,
    pub count: u32,
}

// Error Models
#[derive(Debug, Deserialize, Serialize)]
pub struct SonarQubeError {
    pub errors: Vec<ErrorDetail>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorDetail {
    pub msg: String,
}

// Token Models
#[derive(Debug, Serialize)]
pub struct CreateTokenRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTokenResponse {
    pub token: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenInfo {
    pub name: String,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListTokensResponse {
    pub tokens: Vec<TokenInfo>,
}

// Configuration Models
#[derive(Debug, Clone)]
pub struct SonarQubeConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub timeout_seconds: u64,
}

impl Default for SonarQubeConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:9000".to_string(),
            username: "admin".to_string(),
            password: "Avmmyenbd0!123".to_string(),
            timeout_seconds: 30,
        }
    }
}
