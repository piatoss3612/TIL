mod db;
mod rest;
mod view;

use crate::db::init_db;
use anyhow::{Ok, Result};
use axum::{Extension, Router};
use rest::book_service;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use view::view_service;

fn router(pool: SqlitePool) -> Router {
    Router::new()
        .nest_service("/books", book_service())
        .nest_service("/", view_service())
        .layer(Extension(pool))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let connection_pool = init_db().await?;

    let app = router(connection_pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
