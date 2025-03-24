mod handlers;
mod models;

use axum::{
    Extension, Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use dotenvy::dotenv;
use handlers::posts::{create_post, delete_post, get_post, get_posts, update_post};
use models::{CreateUser, User};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database");

    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/posts", get(get_posts).post(create_post))
        .route(
            "/posts/{id}",
            get(get_post).put(update_post).delete(delete_post),
        )
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn create_user(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        new_user.username,
        new_user.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user))
}
