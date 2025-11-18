pub mod models;
pub mod library;
use crate::library::library::Library;
use std::io::{self, Write};
use sqlx::PgPool;

// Function to get user input as a string
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("Failed to read input");
    val.trim().to_string()
}

// Function to get user input as a u32 number
fn input_u32(prompt: &str) -> u32 {
    loop {
        let s = input(prompt);
        match s.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Enter a valid number."),
        }
    }
}

// Entry point
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // PostgreSQL connection URL
    let database_url = "postgres://postgres:admin@localhost/rust_library";

    // Create Library instance using the URL (not the pool)
    let library = Library::new(database_url).await?;

    loop {
        println!("\n==============================");
        println!("     SCHOOL LIBRARY MENU      ");
        println!("==============================");
        println!("1. Add Student");
        println!("2. Add Teacher");
        println!("3. Add Book");
        println!("4. Borrow Book");
        println!("5. Return Book");
        println!("6. List Available Books");
        println!("7. List Borrowed Books");
        println!("8. List Students");
        println!("9. Exit");
        println!("==============================");

        let choice = input("Choose an option: ");

        match choice.as_str() {
            "1" => {
                let name = input("Enter student name: ");
                library.add_student(name).await?;
                println!("Student added successfully.");
            }
            "2" => {
                let name = input("Enter teacher name: ");
                library.add_teacher(name).await?;
                println!("Teacher added successfully.");
            }
            "3" => {
                let title = input("Enter book title: ");
                let author = input("Enter book author: ");
                let isbn = input("Enter ISBN: ");
                library.add_book(title, author, isbn).await?;
                println!("Book added successfully.");
            }
            "4" => {
                let user_id = input_u32("Enter user ID: ");
                let book_id = input_u32("Enter book ID: ");
                library.borrow_book(user_id as i32, book_id as i32).await?;
            }
            "5" => {
                let book_id = input_u32("Enter book ID to return: ");
                library.return_book(book_id as i32).await?;
            }
            "6" => {
                println!("\nAvailable books:");
                library.list_available_books().await?;
            }
            "7" => {
                println!("\nBorrowed books:");
                library.list_borrowed_books().await?;
            }
            "8" => {
                println!("\nStudents:");
                library.list_students().await?;
            }
            "9" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }

    Ok(())
}
