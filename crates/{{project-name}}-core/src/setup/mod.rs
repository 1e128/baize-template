use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use baizekit::app::config::Config;
use baizekit::app::error::Result;
use baizekit::component::{AxumServiceInfo, Component};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::_db::*;
use crate::service::*;
use self::state::AppState;

mod http;
mod state;


pub struct {{project-name | pascal_case}}Application {
    state: AppState,
    router: Router,
}

impl {{project-name | pascal_case}}Application {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        let state = AppState {  };

        let router = http::new_router(state.clone());
        Self { state, router }
    }

    pub fn services(&self) -> Vec<AxumServiceInfo> {
        vec![
            AxumServiceInfo::new("/{{project-name}}", self.router.clone(), http::ApiDoc::openapi()),
        ]
    }
}

#[async_trait]
impl Component for {{project-name | pascal_case}}Application {
    async fn init(&mut self, _config: &Config, _label: &str) -> Result<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}
