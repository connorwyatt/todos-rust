use async_trait::async_trait;

use crate::todos::data::models::Todo;

#[async_trait]
pub(crate) trait TodosRepository {
    async fn get_todos(&self) -> Vec<Todo>;

    async fn get_todo(&self, todo_id: String) -> Option<Todo>;

    async fn add_todo(&mut self, todo: Todo);

    async fn update_todo(&mut self, todo: Todo);
}
