use uuid::Uuid;
use baizekit::db::curd::*;
use baizekit::derive::{PaginatedFilter, With};
use {{project-name | snake_case}}_sdk::error::Error;

use super::{{entity | pascal_case}};

#[derive(Clone, derive_more::From)]
pub enum FindFilter {
    ByID(Uuid),
}

#[derive(Default, With, PaginatedFilter)]
pub struct SearchFilter {
    #[paginate]
    pub(crate) paginate: Option<Pagination>,
}

pub trait {{entity | pascal_case}}Repository:
    FindTrait<{{entity | pascal_case}}, Error, FindFilter>
    + SearchTrait<{{entity | pascal_case}}, Error, SearchFilter>
    + SearchStreamTrait<{{entity | pascal_case}}, Error, SearchFilter>
    + InsertTrait<{{entity | pascal_case}}, Error>
    + UpdateTrait<{{entity | pascal_case}}, Error>
    + DeleteTrait<{{entity | pascal_case}}, Error>
    + UpsertTrait<{{entity | pascal_case}}, Error>
    + BulkInsertTrait<{{entity | pascal_case}}, Error>
    + BulkUpsertTrait<{{entity | pascal_case}}, Error>
{
}
