mod models;
mod routes;
mod db;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
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
    
    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    tracing::info!("Migrations completed successfully");

    // Build application router
    let app = Router::new()
        // Season routes
        .route("/api/seasons", post(routes::create_season))
        .route("/api/seasons", get(routes::list_seasons))
        .route("/api/seasons/active", get(routes::get_active_season))
        
        // Access key routes
        .route("/api/access-keys", post(routes::create_access_keys))
        .route("/api/access-keys/validate", post(routes::validate_access_key))
        
        // Golfer routes
        .route("/api/golfers", get(routes::list_golfers))
        .route("/api/golfers", post(routes::create_golfer))
        
        // Team routes
        .route("/api/teams", post(routes::create_team))
        .route("/api/teams/:season_id", get(routes::list_teams))
        .route("/api/teams/:team_id/golfers", get(routes::get_team_golfers))
        
        // Tournament routes
        .route("/api/tournaments", post(routes::create_tournament))
        .route("/api/tournaments/:season_id", get(routes::list_tournaments))
        
        // Scores routes
        .route("/api/scores", post(routes::add_hole_scores))
        .route("/api/scores/tournament/:tournament_id", get(routes::get_tournament_scores))
        
        // Leaderboard routes
        .route("/api/leaderboard/:season_id", get(routes::get_season_leaderboard))
        .route("/api/leaderboard/tournament/:tournament_id", get(routes::get_tournament_leaderboard))
        
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
        .fallback_service(ServeDir::new("dist"));

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    tracing::info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}