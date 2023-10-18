use crate::todos::{
    api::models::{self, TodoPatch},
    data::{
        self, in_memory_todos_repository::InMemoryTodosRepository,
        todos_repository::TodosRepository,
    },
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

type SharedRepository = Arc<RwLock<dyn TodosRepository + Send + Sync>>;

pub(crate) fn router() -> Router {
    let shared_repository: SharedRepository =
        Arc::new(RwLock::new(InMemoryTodosRepository::default()));

    Router::new()
        .route("/todos", get(get_todos).post(add_todo))
        .route("/todos/:todo_id", get(get_todo).patch(update_todo))
        .route("/todos/:todo_id/actions/complete", post(complete_todo))
        .layer(Extension(Arc::clone(&shared_repository)))
}

async fn get_todos(Extension(repository): Extension<SharedRepository>) -> impl IntoResponse {
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
    Extension(repository): Extension<SharedRepository>,
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
    Extension(repository): Extension<SharedRepository>,
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
    Extension(repository): Extension<SharedRepository>,
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

async fn complete_todo(
    Extension(repository): Extension<SharedRepository>,
    Path(todo_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = repository
        .read()
        .await
        .get_todo(todo_id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    todo = data::models::Todo {
        is_complete: true,
        completed_at: Some(Utc::now()),
        ..todo
    };

    repository.write().await.update_todo(todo).await;

    Ok(())
}
