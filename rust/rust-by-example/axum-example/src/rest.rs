use axum::{
    extract::{self, Path},
    http::StatusCode,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use sqlx::SqlitePool;

use crate::db::{all_books, book_by_id, Book};

pub fn book_service() -> Router {
    Router::new()
        .route("/", get(get_all_books))
        .route("/:id", get(get_book))
        .route("/add", post(add_book))
        .route("/edit", put(update_book))
        .route("/delete/:id", delete(delete_book))
}

async fn get_all_books(
    Extension(conn): Extension<SqlitePool>,
) -> Result<Json<Vec<Book>>, StatusCode> {
    if let Ok(books) = all_books(&conn).await {
        Ok(Json(books))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn get_book(
    Extension(conn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Book>, StatusCode> {
    if let Ok(book) = book_by_id(&conn, id).await {
        Ok(Json(book))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn add_book(
    Extension(conn): Extension<SqlitePool>,
    extract::Json(book): extract::Json<Book>,
) -> Result<Json<i32>, StatusCode> {
    if let Ok(id) = crate::db::add_book(&conn, &book.title, &book.author).await {
        Ok(Json(id))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn update_book(
    Extension(conn): Extension<SqlitePool>,
    extract::Json(book): extract::Json<Book>,
) -> StatusCode {
    if crate::db::update_book(&conn, &book).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn delete_book(Extension(conn): Extension<SqlitePool>, Path(id): Path<i32>) -> StatusCode {
    if crate::db::delete_book(&conn, id).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
