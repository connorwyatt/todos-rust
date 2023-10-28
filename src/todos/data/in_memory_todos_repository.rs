use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::todos::data::{
    models::Todo,
    todos_repository::{
        RepositoryResult,
        TodosRepository,
    },
};

#[derive(Default)]
pub(crate) struct InMemoryTodosRepository {
    todos: RwLock<HashMap<String, Todo>>,
}

#[async_trait]
impl TodosRepository for InMemoryTodosRepository {
    async fn get_todos(&self) -> RepositoryResult<Vec<Todo>> {
        let todos = self
            .todos
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<Todo>>();

        tracing::debug!("fetched Todos");

        Ok(todos)
    }

    async fn get_todo(&self, todo_id: String) -> RepositoryResult<Option<Todo>> {
        let todo = self.todos.read().await.get(&todo_id).cloned();

        tracing::debug!(todo_id = %&todo_id, "fetched Todo");

        Ok(todo)
    }

    async fn add_todo(&self, todo: Todo) -> RepositoryResult {
        self.todos
            .write()
            .await
            .insert(todo.id.clone(), todo.clone());

        tracing::debug!(todo_id = %&todo.id, "inserted Todo");

        Ok(())
    }

    async fn update_todo(&self, todo: Todo) -> RepositoryResult {
        if !self.todos.read().await.contains_key(&todo.id) {
            panic!("cannot find Todo \"{}\" to update", todo.id);
        }

        self.todos
            .write()
            .await
            .insert(todo.id.clone(), todo.clone());

        tracing::debug!(todo_id = %&todo.id, "updated Todo");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{
        TimeZone,
        Utc,
    };
    use uuid::Uuid;

    use super::InMemoryTodosRepository;
    use crate::todos::data::{
        models::Todo,
        todos_repository::TodosRepository,
    };

    #[tokio::test]
    async fn todos_can_be_added_and_retrieve() {
        let repository = InMemoryTodosRepository::default();

        let todo = Todo::default();

        let add_todo_result = repository.add_todo(todo.clone()).await;

        assert!(add_todo_result.is_ok());

        let fetched_todo_option = repository.get_todo(todo.id.clone()).await;

        assert!(fetched_todo_option.clone().is_ok_and(|x| x.is_some()));

        let fetched_todo = fetched_todo_option
            .expect("already asserted above")
            .expect("already asserted above");

        assert_eq!(&fetched_todo.id, &todo.id);
        assert_eq!(&fetched_todo.title, &todo.title);
        assert_eq!(&fetched_todo.added_at, &todo.added_at);
        assert_eq!(&fetched_todo.is_complete, &todo.is_complete);
        assert_eq!(&fetched_todo.completed_at, &todo.completed_at);
    }

    #[tokio::test]
    async fn todos_can_be_updated() {
        let repository = InMemoryTodosRepository::default();

        let todo = Todo::default();

        let add_todo_result = repository.add_todo(todo.clone()).await;

        assert!(add_todo_result.is_ok());

        let update_todo_result = repository
            .update_todo(Todo {
                id: todo.id.clone(),
                title: String::from("Sweep the ceilings"),
                added_at: todo.added_at,
                is_complete: todo.is_complete,
                completed_at: todo.completed_at,
            })
            .await;

        assert!(update_todo_result.is_ok());

        let fetched_todo_result = repository.get_todo(todo.id.clone()).await;

        assert!(fetched_todo_result.clone().is_ok_and(|x| x.is_some()));

        let fetched_todo = fetched_todo_result
            .expect("already asserted above")
            .expect("already asserted above");

        assert_eq!(&fetched_todo.id, &todo.id);
        assert_eq!(&fetched_todo.title, &String::from("Sweep the ceilings"));
        assert_eq!(&fetched_todo.added_at, &todo.added_at);
        assert_eq!(&fetched_todo.is_complete, &todo.is_complete);
        assert_eq!(&fetched_todo.completed_at, &todo.completed_at);
    }

    impl Default for Todo {
        fn default() -> Self {
            Todo {
                id: Uuid::new_v4().to_string(),
                title: String::from("Clean my room"),
                added_at: Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap(),
                is_complete: true,
                completed_at: Some(Utc.with_ymd_and_hms(2023, 1, 31, 12, 0, 0).unwrap()),
            }
        }
    }
}
