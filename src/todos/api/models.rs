use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::todos::data;

#[derive(Serialize)]
pub(crate) struct Todo {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) added_at: DateTime<Utc>,
    pub(crate) is_complete: bool,
    pub(crate) completed_at: Option<DateTime<Utc>>,
}

impl From<data::models::Todo> for Todo {
    fn from(todo: data::models::Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            added_at: todo.added_at,
            is_complete: todo.is_complete,
            completed_at: todo.completed_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct TodoDefinition {
    pub(crate) title: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TodoPatch {
    pub(crate) title: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct TodoReference {
    pub(crate) id: String,
}
