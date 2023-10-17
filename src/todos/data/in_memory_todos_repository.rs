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
