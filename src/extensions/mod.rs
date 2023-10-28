use std::sync::Arc;

use axum::{
    Extension,
    Router,
};
use sqlx::{
    postgres::PgPoolOptions,
    Pool,
    Postgres,
};

use crate::todos::data::{
    in_memory_todos_repository::InMemoryTodosRepository,
    postgres_todos_repository::PostgresTodosRepository,
    todos_repository::TodosRepository,
};

pub(crate) async fn add(router: Router) -> Router {
    let use_in_memory_repositories =
        std::env::var("USE_IN_MEMORY_REPOSITORIES").map_or(false, |x| {
            x.parse()
                .expect("could not parse USE_IN_MEMORY_REPOSITORIES")
        });

    let pool: Option<Arc<Pool<Postgres>>> = if !use_in_memory_repositories {
        let pool = Arc::new(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(
                    &std::env::var("DATABASE_URL")
                        .expect("missing DATABASE_URL environment variable"),
                )
                .await
                .unwrap(),
        );

        Some(pool)
    } else {
        None
    };

    let todos_repository: Arc<dyn TodosRepository> = match pool {
        None => Arc::new(InMemoryTodosRepository::default()),
        Some(pool) => Arc::new(PostgresTodosRepository::new(Arc::clone(&pool))),
    };

    router.layer(Extension(Arc::clone(&todos_repository)))
}
