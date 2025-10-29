use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
