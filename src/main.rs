mod models;
mod routes;
mod db;
mod auth;

use axum::{
    Router,
    routing::{get, post, put},
    middleware,
};
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use std::net::SocketAddr;
use std::str::FromStr;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
    cors::CorsLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "feathers_and_fairways=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create SQLite database pool
    tracing::info!("Connecting to database...");
    let options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .pragma("journal_mode", "WAL")
        .pragma("foreign_keys", "ON");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    tracing::info!("Successfully connected to database");

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    tracing::info!("Migrations completed successfully");

    // Build application router
    let app = Router::new()
        // Public routes - Access key validation
        .route("/api/access-keys/validate", post(routes::validate_access_key))

        // Public routes - Golfer routes
        .route("/api/golfers", get(routes::list_golfers))
        .route("/api/golfers/tournament/:tournament_id", get(routes::list_golfers_for_tournament))

        // Public routes - Team routes
        .route("/api/teams", get(routes::list_teams_by_tournament).post(routes::create_team))
        .route("/api/teams/update", post(routes::update_team))
        .route("/api/teams/:team_id/golfers", get(routes::get_team_golfers))

        // Public routes - Tournament routes
        .route("/api/tournaments", get(routes::list_all_tournaments))
        .route("/api/tournaments/completed", get(routes::list_completed_tournaments))

        // Public routes - Scores routes
        .route("/api/scores/tournament/:tournament_id", get(routes::get_tournament_scores))

        // Public routes - Leaderboard routes
        .route("/api/leaderboard/tournament/:tournament_id", get(routes::get_tournament_leaderboard))
        .route("/api/leaderboard/tournament/:tournament_id/teams", get(routes::get_tournament_team_leaderboard))
        .route("/api/tournaments/:tournament_id/stats", get(routes::get_tournament_stats))

        // Admin authentication
        .route("/api/admin/login", post(routes::admin_login))

        // Protected admin routes (these will have middleware applied)
        .nest("/api/admin", Router::new()
            .route("/access-keys", post(routes::create_access_keys))
            .route("/golfers", post(routes::create_golfer))
            .route("/golfers/paste", post(routes::paste_golfers))
            .route("/tournaments", post(routes::create_tournament))
            .route("/scores", post(routes::add_hole_scores))
            .route("/tournaments/:tournament_id/teams", get(routes::list_teams_for_tournament))
            .route("/tournaments/:tournament_id/espn-field", post(routes::fetch_espn_field))
            .route("/tournaments/:tournament_id/groups", post(routes::save_tournament_groups))
            .route("/teams/:team_id/golfers", put(routes::admin_update_team_golfers))
            .route("/stats", get(routes::get_admin_stats))
            .route("/tournaments/import/preview", post(routes::import_preview))
            .route("/tournaments/import/espn-preview", post(routes::import_espn_preview))
            .route("/tournaments/import/commit", post(routes::import_commit))
            .route("/tournaments/:tournament_id/scores/refresh", post(routes::refresh_scores))
            .layer(middleware::from_fn(auth::admin_auth_middleware))
        )

        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
        .fallback_service(ServeDir::new("dist"));

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "41549".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
