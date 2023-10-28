use chrono::{
    DateTime,
    Utc,
};

#[derive(Clone, Debug)]
pub(crate) struct Todo {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) added_at: DateTime<Utc>,
    pub(crate) is_complete: bool,
    pub(crate) completed_at: Option<DateTime<Utc>>,
}
