pub(crate) mod server;
pub(crate) mod todos;

use std::sync::Arc;

use axum::{Extension, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::todos::{
    api::routes,
    data::{
        in_memory_todos_repository::InMemoryTodosRepository,
        postgres_todos_repository::PostgresTodosRepository,
        todos_repository::TodosRepository,
    },
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file is missing");

    let file_appender = tracing_appender::rolling::hourly("logs", "logs");
    let (non_blocking_file_appender, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug,hyper=warn".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(non_blocking_file_appender),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .init();

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

    let app = Router::new()
        .merge(routes::router())
        .layer(Extension(Arc::clone(&todos_repository)));

    server::start(app).await;
}
