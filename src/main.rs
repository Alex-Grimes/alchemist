use axum::{Router, routing::get};
use tracing::{Level, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();

    println!("Hello, world!");
}
