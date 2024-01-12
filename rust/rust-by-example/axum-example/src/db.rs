use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as, Row, SqlitePool};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

pub async fn init_db() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

pub async fn all_books(pool: &SqlitePool) -> Result<Vec<Book>> {
    if let Some(books) = CACHE.all_books().await {
        return Ok(books);
    }

    let books = query_as::<_, Book>("SELECT * FROM books ORDER BY title,author")
        .fetch_all(pool)
        .await?;

    CACHE.refresh(books.clone()).await;

    Ok(books)
}

pub async fn book_by_id(pool: &SqlitePool, id: i32) -> Result<Book> {
    let book = query_as::<_, Book>("SELECT * FROM books WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(book)
}

pub async fn add_book<S: ToString>(pool: &SqlitePool, title: S, author: S) -> Result<i32> {
    let title = title.to_string();
    let author = author.to_string();
    let id = sqlx::query("INSERT INTO books (title, author) VALUES ($1, $2) RETURNING id")
        .bind(title)
        .bind(author)
        .fetch_one(pool)
        .await?
        .get(0);

    CACHE.invalidate().await;

    Ok(id)
}

pub async fn update_book(pool: &SqlitePool, book: &Book) -> Result<()> {
    sqlx::query("UPDATE books SET title=$1, author=$2 WHERE id=$3")
        .bind(&book.title)
        .bind(&book.author)
        .bind(book.id)
        .execute(pool)
        .await?;

    CACHE.invalidate().await;

    Ok(())
}

pub async fn delete_book(pool: &SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM books WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await?;

    CACHE.invalidate().await;

    Ok(())
}

struct BookCache {
    all_books: RwLock<Option<Vec<Book>>>,
}

impl BookCache {
    fn new() -> Self {
        Self {
            all_books: RwLock::new(None),
        }
    }

    async fn all_books(&self) -> Option<Vec<Book>> {
        let lock = self.all_books.read().await;
        lock.clone()
    }

    async fn refresh(&self, books: Vec<Book>) {
        let mut lock = self.all_books.write().await;
        *lock = Some(books);
    }

    async fn invalidate(&self) {
        let mut lock = self.all_books.write().await;
        *lock = None;
    }
}

static CACHE: Lazy<BookCache> = Lazy::new(BookCache::new);
