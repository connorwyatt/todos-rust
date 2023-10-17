use std::sync::Arc;

use crate::todos::{
    api::models,
    data::{self, in_memory_todos_repository::InMemoryTodosRepository},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::models::TodoPatch;

type SharedRepository = Arc<RwLock<InMemoryTodosRepository>>;

pub(crate) fn router() -> Router {
    let shared_repository = SharedRepository::default();

    Router::new()
        .route("/todos", get(get_todos).post(add_todo))
        .route("/todos/:todo_id", get(get_todo).patch(update_todo))
        .with_state(Arc::clone(&shared_repository))
}

async fn get_todos(State(repository): State<SharedRepository>) -> impl IntoResponse {
    let todos = repository
        .read()
        .await
        .get_todos()
        .await
        .iter()
        .cloned()
        .map(models::Todo::from)
        .collect::<Vec<_>>();

    Json(todos)
}

async fn get_todo(
    State(repository): State<SharedRepository>,
    Path(todo_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .read()
        .await
        .get_todo(todo_id)
        .await
        .map(models::Todo::from)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

async fn add_todo(
    State(repository): State<SharedRepository>,
    Json(definition): Json<models::TodoDefinition>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();

    let todo = data::models::Todo {
        id: id.clone(),
        title: definition.title,
        added_at: Utc::now(),
        is_complete: false,
        completed_at: None,
    };

    repository.write().await.add_todo(todo).await;

    Json(models::TodoReference { id })
}

async fn update_todo(
    State(repository): State<SharedRepository>,
    Path(todo_id): Path<String>,
    Json(patch): Json<TodoPatch>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = repository
        .read()
        .await
        .get_todo(todo_id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    if let Some(title) = patch.title {
        todo = data::models::Todo { title, ..todo };
    }

    repository.write().await.update_todo(todo).await;

    Ok(())
}
