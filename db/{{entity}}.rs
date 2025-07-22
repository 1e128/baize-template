use std::sync::Arc;

use baizekit::db::curd::*;
use baizekit::derive::Curd;
use {{project-name | snake_case}}_sdk::error::{Error, Result};
use sea_orm::*;
use sea_orm::sea_query::OnConflict;

use super::entities::{{db_prefix}}_{{entity}}::*;
use crate::domain::{{entity}}::*;

impl From<Model> for {{entity | pascal_case}} {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at.and_utc(),
            updated_at: model.updated_at.and_utc(),
        }
    }
}

impl From<{{entity | pascal_case}}> for ActiveModel {
    fn from(entity: {{entity | pascal_case}}) -> Self {
        Self {
            id: Set(entity.id),
            created_at: Set(entity.created_at.naive_utc()),
            updated_at: Set(entity.updated_at.naive_utc()),
        }
    }
}

fn find_select_fn(filter: FindFilter) -> Select<Entity> {
    match filter {
        FindFilter::ByID(id) => Entity::find().filter(Column::Id.eq(id)),
    }
}

fn search_select_fn(_filter: SearchFilter) -> Select<Entity> {
    let mut select = Entity::find();

    select = select.order_by_desc(Column::Id);
    select
}

fn on_conflict_fn() -> OnConflict {
    OnConflict::new().do_nothing().to_owned()
}

#[derive(Curd)]
#[curd(
    db_entity = Entity,
    domain_entity = {{entity | pascal_case}},
    error = Error,
    find(filter = FindFilter, select_fn = find_select_fn),
    search(filter = SearchFilter, select_fn = search_select_fn),
    stream_search(filter = SearchFilter, select_fn = search_select_fn),
    insert,
    update,
    delete,
    upsert(on_conflict_fn = on_conflict_fn),
    bulk_insert,
    bulk_upsert(on_conflict_fn = on_conflict_fn),
)]
pub struct {{entity | pascal_case}}RepositoryImpl {
    #[curd(db)]
    db: Arc<DatabaseConnection>,
}

impl {{entity | pascal_case}}Repository for {{entity | pascal_case}}RepositoryImpl {}
