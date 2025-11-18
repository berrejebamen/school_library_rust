Absolutely! Here's a **complete `README.md`** that includes everything: project purpose, folder structure, setup instructions, running instructions, example commands, features, and notes—all in **one file** ready to push to GitHub:

```markdown
# School Library Management System (Rust + PostgreSQL)

This project is a **console-based school library management system** implemented in **Rust** using **SQLx** for PostgreSQL database operations. The system allows adding students, teachers, books, borrowing and returning books, and listing available and borrowed books. All data is persisted in a PostgreSQL database.

---

## 📝 Project Purpose

The purpose of this project is to demonstrate:

- Rust programming for building a CLI application.
- Integration with PostgreSQL for persistent storage.
- Asynchronous programming with `tokio` and `async`/`await`.
- Database CRUD operations using SQLx.
- Basic object-oriented design with structs and enums for Users and Library management.

---

## 📂 Folder Structure

```

school_library/
├── Cargo.toml                 # Rust project manifest
├── Cargo.lock
├── README.md
├── src/
│   ├── main.rs                # Entry point of the application (menu + CLI)
│   ├── library.rs             # Library struct with all database operations
│   └── models/
│       ├── mod.rs             # Models module
│       ├── book.rs            # Book struct
│       ├── student.rs         # Student struct
│       ├── teacher.rs         # Teacher struct
│       └── borrow_record.rs   # BorrowRecord struct
└── example
    |── example.rs # small example how to connect to a sql database using rust
````

---

## ⚙️ Prerequisites

- Rust (latest stable version)
- PostgreSQL (local or remote server)
- `sqlx-cli` (optional, for migrations and queries)

---

## 🏗️ Setup Instructions

1. **Clone the repository**
```bash
git clone https://github.com/berrejebamen/school_library_rust.git
cd school_library_rust
````

2. **Set up PostgreSQL database**

```sql
CREATE DATABASE rust_library;
```

3. **Update database URL in `main.rs`**

```rust
let database_url = "postgres://postgres:YOUR_PASSWORD@localhost/rust_library";
```

Replace `YOUR_PASSWORD` with your PostgreSQL password.

4. **Install Rust dependencies**

```bash
cargo build
```

5. **Optional: Install `sqlx-cli` for migrations**

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

---

## 🚀 Running the Application

Run the CLI application with:

```bash
cargo run
```

You will see a menu:

```
==============================
     SCHOOL LIBRARY MENU      
==============================
1. Add Student
2. Add Teacher
3. Add Book
4. Borrow Book
5. Return Book
6. List Available Books
7. List Borrowed Books
8. List Students
9. Exit
==============================
```

### Menu Options

* **Add Student / Teacher / Book**: Adds entries to the database.
* **Borrow / Return Book**: Updates book availability and borrow records.
* **List Books / Students / Teachers**: Prints entries from the database.

---

## 🧩 Example Commands

You can test the system directly from the menu:

1. **Add a Student**

```
Enter student name: Alice
Student added successfully.
```

2. **Add a Book**

```
Enter book title: Rust Programming
Enter book author: John Doe
Enter ISBN: 12345
Book added successfully.
```

3. **Borrow a Book**

```
Enter user ID: 1
Enter book ID: 1
User 1 borrowed book 1
```

4. **Return a Book**

```
Enter book ID to return: 1
Book 1 returned successfully!
```

5. **List Available Books**

```
[1] Rust Programming by John Doe
```

---

## ⚡ Features

* Persistent storage using PostgreSQL.
* Automatic table creation if tables do not exist.
* Console-based menu for easy interaction.
* Asynchronous database operations for better performance.
* Basic user management (Students and Teachers) and book management.

---

## ⚠️ Notes

* Ensure PostgreSQL server is running before starting the application.
* Database credentials in `main.rs` must match your local setup.
* The system automatically creates tables if they are missing (`users`, `books`, `borrow_records`).

---

## 📄 License

MIT License – feel free to use and modify this project.

```

---

If you want, I can also **write a `.gitignore` file** for Rust + PostgreSQL so you can push safely to GitHub without committing target files, `.env`, or database-related files.  

Do you want me to do that next?
```
