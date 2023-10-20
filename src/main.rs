pub(crate) mod latency;
pub(crate) mod middleware;
pub(crate) mod todos;

use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{Extension, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
    ServiceBuilderExt,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    latency::Latency,
    middleware::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse},
    todos::data::{
        in_memory_todos_repository::InMemoryTodosRepository,
        postgres_todos_repository::PostgresTodosRepository,
        todos_repository::TodosRepository,
    },
};

#[tokio::main]
async fn main() {
    let start = Instant::now();

    dotenvy::dotenv().expect(".env file is missing");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todos=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let middleware = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default())
                .on_request(DefaultOnRequest::default())
                .on_response(DefaultOnResponse::default()),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .propagate_x_request_id()
        .map_response_body(axum::body::boxed)
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .compression();

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
        .merge(todos::api::routes::router())
        .layer(Extension(Arc::clone(&todos_repository)))
        .layer(middleware);

    let server = axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service());
    tracing::debug!(
        "listening on {}, started in {}",
        server.local_addr(),
        Latency::new(start.elapsed())
    );
    server.await.unwrap();
}
