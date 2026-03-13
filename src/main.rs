mod db;
mod handlers;
mod models;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_livereload::LiveReloadLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use models::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).await;
    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(handlers::dashboard_handler))
        .route(
            "/tasks",
            get(handlers::tasks_handler).post(handlers::tasks_create_action),
        )
        .route("/tasks/create", get(handlers::tasks_create_handler))
        .route("/tasks/{id}/edit", get(handlers::tasks_edit_handler))
        .route(
            "/tasks/{id}",
            post(handlers::tasks_update_action).delete(handlers::tasks_delete_action),
        )
        .nest_service("/public", ServeDir::new("public"))
        .layer(LiveReloadLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!(
        "サーバーを起動しました: http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
