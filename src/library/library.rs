use chrono::Local;
use sqlx::{PgPool, Row};

use crate::models::book::Book;
use crate::models::borrow_record::BorrowRecord;
use crate::models::student::Student;
use crate::models::teacher::Teacher;

pub enum UserType {
    Student(Student),
    Teacher(Teacher),
}

pub struct Library {
    pool: PgPool,
}

impl Library {
    // -------------------------
    // Create a new Library object and ensure tables exist
    // -------------------------
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
       let pool = PgPool::connect(database_url).await?;

    // Create users table
        sqlx::query(
           r#"
            CREATE TABLE IF NOT EXISTS users (
              id SERIAL PRIMARY KEY,
              name TEXT NOT NULL,
              type TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await?;

    // Create books table
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS books (
               id SERIAL PRIMARY KEY,
               title TEXT NOT NULL,
               author TEXT NOT NULL,
               isbn TEXT NOT NULL,
               is_available BOOLEAN NOT NULL DEFAULT TRUE
        );
        "#
    )
    .execute(&pool)
    .await?;

    // Create borrow_records table
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS borrow_records (
               id SERIAL PRIMARY KEY,
               user_id INT REFERENCES users(id),
               book_id INT REFERENCES books(id),
               borrow_date DATE NOT NULL,
               return_date DATE
        );
        "#
    )
    .execute(&pool)
    .await?;

    Ok(Self { pool })
}

    // -------------------------
    // Add a new student
    // -------------------------
    pub async fn add_student(&self, name: String) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (name, type) VALUES ($1, 'student')")
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // -------------------------
    // Add a new teacher
    // -------------------------
    pub async fn add_teacher(&self, name: String) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (name, type) VALUES ($1, 'teacher')")
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // -------------------------
    // Add a new book
    // -------------------------
    pub async fn add_book(&self, title: String, author: String, isbn: String) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO books (title, author, isbn) VALUES ($1, $2, $3)")
            .bind(title)
            .bind(author)
            .bind(isbn)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // -------------------------
    // List all students
    // -------------------------
    pub async fn list_students(&self) -> Result<(), sqlx::Error> {
        let rows = sqlx::query("SELECT id, name FROM users WHERE type='student'")
            .fetch_all(&self.pool)
            .await?;

        println!("\nList of Students:");
        for row in rows {
            let id: i32 = row.get("id");
            let name: String = row.get("name");
            println!("[{}] {}", id, name);
        }
        Ok(())
    }

    // -------------------------
    // List all teachers
    // -------------------------
    pub async fn list_teachers(&self) -> Result<(), sqlx::Error> {
        let rows = sqlx::query("SELECT id, name FROM users WHERE type='teacher'")
            .fetch_all(&self.pool)
            .await?;

        println!("\nList of Teachers:");
        for row in rows {
            let id: i32 = row.get("id");
            let name: String = row.get("name");
            println!("[{}] {}", id, name);
        }
        Ok(())
    }

    // -------------------------
    // Borrow a book
    // -------------------------
    pub async fn borrow_book(&self, user_id: i32, book_id: i32) -> Result<(), sqlx::Error> {
        let available: Option<bool> = sqlx::query_scalar("SELECT is_available FROM books WHERE id=$1")
            .bind(book_id)
            .fetch_optional(&self.pool)
            .await?;

        match available {
            Some(true) => {
                sqlx::query("UPDATE books SET is_available=false WHERE id=$1")
                    .bind(book_id)
                    .execute(&self.pool)
                    .await?;

                sqlx::query("INSERT INTO borrow_records (user_id, book_id, borrow_date) VALUES ($1, $2, $3)")
                    .bind(user_id)
                    .bind(book_id)
                    .bind(Local::now().format("%Y-%m-%d").to_string())
                    .execute(&self.pool)
                    .await?;

                println!("User {} borrowed book {}", user_id, book_id);
            }
            Some(false) => println!("Book {} is already borrowed!", book_id),
            None => println!("Book not found."),
        }
        Ok(())
    }

    // -------------------------
    // Return a book
    // -------------------------
    pub async fn return_book(&self, book_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE books SET is_available=true WHERE id=$1")
            .bind(book_id)
            .execute(&self.pool)
            .await?;

        sqlx::query("UPDATE borrow_records SET return_date=$1 WHERE book_id=$2 AND return_date IS NULL")
            .bind(Local::now().format("%Y-%m-%d").to_string())
            .bind(book_id)
            .execute(&self.pool)
            .await?;

        println!("Book {} returned successfully!", book_id);
        Ok(())
    }

    // -------------------------
    // List all available books
    // -------------------------
    pub async fn list_available_books(&self) -> Result<(), sqlx::Error> {
        let rows = sqlx::query("SELECT id, title, author FROM books WHERE is_available=true")
            .fetch_all(&self.pool)
            .await?;

        println!("\nAvailable Books:");
        for row in rows {
            let id: i32 = row.get("id");
            let title: String = row.get("title");
            let author: String = row.get("author");
            println!("[{}] {} by {}", id, title, author);
        }
        Ok(())
    }

    // -------------------------
    // List all borrowed books
    // -------------------------
    pub async fn list_borrowed_books(&self) -> Result<(), sqlx::Error> {
        let rows = sqlx::query(
            "SELECT b.id as book_id, b.title, u.id as user_id, u.name as user_name, br.borrow_date
             FROM borrow_records br
             JOIN books b ON br.book_id=b.id
             JOIN users u ON br.user_id=u.id
             WHERE br.return_date IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;

        println!("\nBorrowed Books:");
        for row in rows {
            let book_id: i32 = row.get("book_id");
            let title: String = row.get("title");
            let user_id: i32 = row.get("user_id");
            let user_name: String = row.get("user_name");
            let borrow_date: String = row.get("borrow_date");

            println!("Book {} ({}) borrowed by {} [{}] on {}", book_id, title, user_name, user_id, borrow_date);
        }
        Ok(())
    }
}
