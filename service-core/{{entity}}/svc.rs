use std::sync::Arc;

use baizekit::api::Page;
use {{project-name | snake_case}}_sdk::service::{{entity}}::*;
use {{project-name | snake_case}}_sdk::error::Result;

use crate::domain::{{entity}}::*;

impl From<&QueryCommand> for FindFilter {
    fn from(cmd: &QueryCommand) -> Self {
        todo!()
    }
}

impl From<&SearchCommand> for SearchFilter {
    fn from(cmd: &SearchCommand) -> Self {
        todo!()
    }
}

impl From<{{entity | pascal_case}}> for {{entity | pascal_case}}DTO {
    fn from(entity: {{entity | pascal_case}}) -> Self {
        todo!()
    }
}

pub struct {{entity | pascal_case}}ServiceImpl {
    repo: Arc<dyn {{entity | pascal_case}}Repository>,
}

impl {{entity | pascal_case}}ServiceImpl {
    pub fn new(repo: Arc<dyn {{entity | pascal_case}}Repository>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl {{entity | pascal_case}}Service for {{entity | pascal_case}}ServiceImpl {
    async fn query(&self, cmd: QueryCommand) -> Result<{{entity | pascal_case}}DTO> {
        todo!()
    }

    async fn search(&self, cmd: SearchCommand) -> Result<Page<{{entity | pascal_case}}DTO>> {
        let (data, total, _) = self.repo.search(SearchFilter::from(&cmd)).await?;

        Ok(Page {
            total,
            current: cmd.page.unwrap_or_default(),
            size: cmd.size.unwrap_or_default(),
            data: data.into_iter().map({{entity | pascal_case}}DTO::from).collect(),
        })
    }

    async fn add(&self, cmd: AddCommand) -> Result<{{entity | pascal_case}}DTO> {
        todo!()
    }

    async fn update(&self, cmd: UpdateCommand) -> Result<{{entity | pascal_case}}DTO> {
        todo!()
    }

}