use baizekit::api::Page;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::error::Result;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct {{entity | pascal_case}}DTO {
    pub id: Uuid,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct QueryCommand {
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
#[serde(default)]
pub struct SearchCommand {
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Default for SearchCommand {
    fn default() -> Self {
        Self { page: Some(1), size: Some(10) }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct AddCommand {
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateCommand {
    #[serde(skip)]
    pub id: Uuid,
}

#[async_trait::async_trait]
pub trait {{entity | pascal_case}}Service: Send + Sync {
    async fn query(&self, cmd: QueryCommand) -> Result<{{entity | pascal_case}}DTO>;

    async fn search(&self, cmd: SearchCommand) -> Result<Page<{{entity | pascal_case}}DTO>>;

    async fn add(&self, cmd: AddCommand) -> Result<{{entity | pascal_case}}DTO>;

    async fn update(&self, cmd: UpdateCommand) -> Result<{{entity | pascal_case}}DTO>;
}
