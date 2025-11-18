// example.rs
use sqlx::{PgPool, Row};
use tokio;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connect to PostgreSQL (replace password with yours)
    let database_url = "postgres://postgres:admin@localhost/rust_library";
    let pool = PgPool::connect(database_url).await?;

    // Create a table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS books (
            id SERIAL PRIMARY KEY,
            title VARCHAR(100) NOT NULL,
            author VARCHAR(100) NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    // Insert a book
    sqlx::query("INSERT INTO books (title, author) VALUES ($1, $2)")
        .bind("The Rust Book")
        .bind("Steve Klabnik")
        .execute(&pool)
        .await?;

    // Read books
    let rows = sqlx::query("SELECT id, title, author FROM books")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        let author: String = row.get("author");
        println!("[{}] {} by {}", id, title, author);
    }

    Ok(())
}
