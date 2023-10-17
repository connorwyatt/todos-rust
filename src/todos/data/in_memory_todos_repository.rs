use crate::todos::data::models::Todo;
use std::collections::HashMap;

pub(crate) struct InMemoryTodosRepository {
    todos: HashMap<String, Todo>,
}

impl Default for InMemoryTodosRepository {
    fn default() -> Self {
        Self {
            todos: HashMap::new(),
        }
    }
}

impl InMemoryTodosRepository {
    pub(crate) async fn get_todos(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect::<Vec<Todo>>()
    }

    pub(crate) async fn get_todo(&self, todo_id: String) -> Option<Todo> {
        self.todos.get(&todo_id).cloned()
    }

    pub(crate) async fn add_todo(&mut self, todo: Todo) {
        self.todos.insert(todo.id.clone(), todo);
    }

    pub(crate) async fn update_todo(&mut self, todo: Todo) {
        if !self.todos.contains_key(&todo.id) {
            panic!("cannot find Todo \"{}\" to update", todo.id);
        }

        self.todos.insert(todo.id.clone(), todo);
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryTodosRepository;
    use crate::todos::data::models::Todo;
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    #[test]
    fn todos_can_be_added_and_retrieve() {
        let mut repository = InMemoryTodosRepository::default();

        let todo = Todo::default();

        let add_todo_future = repository.add_todo(todo.clone());

        tokio_test::block_on(add_todo_future);

        let get_todo_future = repository.get_todo(todo.id.clone());

        let fetched_todo_option = tokio_test::block_on(get_todo_future);

        assert!(fetched_todo_option.is_some());

        let fetched_todo = fetched_todo_option.expect("already asserted above");

        assert_eq!(&fetched_todo.id, &todo.id);
        assert_eq!(&fetched_todo.title, &todo.title);
        assert_eq!(&fetched_todo.added_at, &todo.added_at);
        assert_eq!(&fetched_todo.is_complete, &todo.is_complete);
        assert_eq!(&fetched_todo.completed_at, &todo.completed_at);
    }

    #[test]
    fn todos_can_be_updated() {
        let mut repository = InMemoryTodosRepository::default();

        let todo = Todo::default();

        let add_todo_future = repository.add_todo(todo.clone());

        tokio_test::block_on(add_todo_future);

        let update_todo_future = repository.update_todo(Todo {
            id: todo.id.clone(),
            title: String::from("Sweep the ceilings"),
            added_at: todo.added_at.clone(),
            is_complete: todo.is_complete.clone(),
            completed_at: todo.completed_at.clone(),
        });

        tokio_test::block_on(update_todo_future);

        let get_todo_future = repository.get_todo(todo.id.clone());

        let fetched_todo_option = tokio_test::block_on(get_todo_future);

        assert!(fetched_todo_option.is_some());

        let fetched_todo = fetched_todo_option.expect("already asserted above");

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
                added_at: Utc.with_ymd_and_hms(2023, 01, 01, 12, 00, 00).unwrap(),
                is_complete: true,
                completed_at: Some(Utc.with_ymd_and_hms(2023, 01, 31, 12, 00, 00).unwrap()),
            }
        }
    }
}
