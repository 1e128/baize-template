use chrono::{DateTime, Utc};

pub struct {{entity | pascal_case}} {
    /// ID
    pub(crate) id: uuid::Uuid,
    /// 创建时间
    pub(crate) created_at: DateTime<Utc>,
    /// 更新时间
    pub(crate) updated_at: DateTime<Utc>,
}

