use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{
        get,
        post,
    },
    Extension,
    Json,
    Router,
};
use chrono::Utc;
use uuid::Uuid;

use crate::todos::{
    api::models::{
        self,
        TodoPatch,
    },
    data::{
        self,
        todos_repository::TodosRepository,
    },
};

type TodosRepositoryExtension = Arc<dyn TodosRepository>;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/todos", get(get_todos).post(add_todo))
        .route("/todos/:todo_id", get(get_todo).patch(update_todo))
        .route("/todos/:todo_id/actions/complete", post(complete_todo))
}

async fn get_todos(
    Extension(repository): Extension<TodosRepositoryExtension>,
) -> Result<impl IntoResponse, StatusCode> {
    let todos = repository
        .get_todos()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .iter()
        .cloned()
        .map(models::Todo::from)
        .collect::<Vec<_>>();

    Ok(Json(todos))
}

async fn get_todo(
    Extension(repository): Extension<TodosRepositoryExtension>,
    Path(todo_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .get_todo(todo_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(models::Todo::from)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

async fn add_todo(
    Extension(repository): Extension<TodosRepositoryExtension>,
    Json(definition): Json<models::TodoDefinition>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = Uuid::new_v4().to_string();

    let todo = data::models::Todo {
        id: id.clone(),
        title: definition.title,
        added_at: Utc::now(),
        is_complete: false,
        completed_at: None,
    };

    repository
        .add_todo(todo)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(models::TodoReference { id }))
}

async fn update_todo(
    Extension(repository): Extension<TodosRepositoryExtension>,
    Path(todo_id): Path<String>,
    Json(patch): Json<TodoPatch>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = repository
        .get_todo(todo_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    if let Some(title) = patch.title {
        todo = data::models::Todo { title, ..todo };
    }

    repository
        .update_todo(todo)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn complete_todo(
    Extension(repository): Extension<TodosRepositoryExtension>,
    Path(todo_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = repository
        .get_todo(todo_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    todo = data::models::Todo {
        is_complete: true,
        completed_at: Some(Utc::now()),
        ..todo
    };

    repository
        .update_todo(todo)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
