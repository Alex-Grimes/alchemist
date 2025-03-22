use axum::{Extension, Router, extract::Path, http::StatusCode, routing::get, serve::Listener};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, pool, postgres::PgPoolOptions, types::Json};
use tracing::{Level, info};
use tracing_subscriber;

#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    user_id: Option<i32>,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database");

    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/", get(root))
        .route("/posts", get(get_posts))
        .route("/posts/:id", get(get_post))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_posts(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(posts))
}

async fn get_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, body FROM posts Where id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(post))
}
