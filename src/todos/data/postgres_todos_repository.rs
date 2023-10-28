use std::sync::Arc;

use async_trait::async_trait;
use chrono::{
    DateTime,
    Utc,
};
use sqlx::{
    Pool,
    Postgres,
};

use crate::todos::data::{
    models::Todo,
    todos_repository::{
        RepositoryResult,
        TodosRepository,
    },
};

#[derive(Clone, sqlx::FromRow)]
struct TodoRow {
    id: String,
    title: String,
    added_at: DateTime<Utc>,
    is_complete: bool,
    completed_at: Option<DateTime<Utc>>,
}

impl From<TodoRow> for Todo {
    fn from(todo: TodoRow) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            added_at: todo.added_at,
            is_complete: todo.is_complete,
            completed_at: todo.completed_at,
        }
    }
}

pub(crate) struct PostgresTodosRepository {
    pool: Arc<Pool<Postgres>>,
}

impl PostgresTodosRepository {
    pub(crate) fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodosRepository for PostgresTodosRepository {
    async fn get_todos(&self) -> RepositoryResult<Vec<Todo>> {
        let query =
            sqlx::query_as::<_, TodoRow>("select * from public.todos order by added_at desc");

        let result = query.fetch_all(&*self.pool).await;

        let rows = match result {
            Err(err) => {
                tracing::error!("an error occurred while trying to get Todos: {}", err);
                return Err(());
            }
            Ok(result) => result,
        };

        let todos = rows
            .iter()
            .cloned()
            .map(|x| x.into())
            .collect::<Vec<Todo>>();

        Ok(todos)
    }

    async fn get_todo(&self, todo_id: String) -> RepositoryResult<Option<Todo>> {
        let query =
            sqlx::query_as::<_, TodoRow>("select * from public.todos where id = $1 limit 1;")
                .bind(todo_id.clone());

        let result = query.fetch_optional(&*self.pool).await;

        let row = match result {
            Err(err) => {
                tracing::error!(
                    "an error occurred while trying to get Todo {}: {}",
                    todo_id,
                    err
                );
                return Err(());
            }
            Ok(result) => result,
        };

        let todo = row.map(|x| x.into());

        Ok(todo)
    }

    async fn add_todo(&self, todo: Todo) -> RepositoryResult {
        let todo_id = todo.id.clone();

        let query = sqlx::query("insert into public.todos (id, title, added_at, is_complete, completed_at) values ($1, $2, $3, $4, $5);")
            .bind(todo.id)
            .bind(todo.title)
            .bind(todo.added_at)
            .bind(todo.is_complete)
            .bind(todo.completed_at);

        let result = query.execute(&*self.pool).await;

        let result = match result {
            Err(err) => {
                tracing::error!("an error occurred while trying to insert Todo: {}", err);
                return Err(());
            }
            Ok(result) => result,
        };

        if result.rows_affected() != 1 {
            tracing::error!(
                "an error occurred while trying to insert Todo: a Todo already exists with ID {}",
                todo_id
            );
            return Err(());
        }

        Ok(())
    }

    async fn update_todo(&self, todo: Todo) -> RepositoryResult {
        let todo_id = todo.id.clone();

        let query = sqlx::query("update public.todos set title = $2, added_at = $3, is_complete = $4, completed_at = $5 where id = $1;")
            .bind(todo.id)
            .bind(todo.title)
            .bind(todo.added_at)
            .bind(todo.is_complete)
            .bind(todo.completed_at);

        let result = query.execute(&*self.pool).await;

        let result = match result {
            Err(err) => {
                tracing::error!(
                    "an error occurred while trying to update Todo {}: {}",
                    todo_id,
                    err
                );
                return Err(());
            }
            Ok(result) => result,
        };

        if result.rows_affected() != 1 {
            tracing::error!(
                "an error occurred while trying to update Todo: there is no Todo with ID {}",
                todo_id
            );
            return Err(());
        }

        Ok(())
    }
}
