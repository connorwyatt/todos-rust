use async_trait::async_trait;

use crate::todos::data::models::Todo;

pub(crate) type RepositoryResult<T = ()> = Result<T, ()>;

#[async_trait]
pub(crate) trait TodosRepository: Send + Sync {
    async fn get_todos(&self) -> RepositoryResult<Vec<Todo>>;

    async fn get_todo(&self, todo_id: String) -> RepositoryResult<Option<Todo>>;

    async fn add_todo(&self, todo: Todo) -> RepositoryResult;

    async fn update_todo(&self, todo: Todo) -> RepositoryResult;
}
