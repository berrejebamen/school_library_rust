pub mod user;
// ----------------
// This tells Rust: "We have a module named `user` in this project."
// A module is like a little box where we put related code together.
// In this case, `user.rs` will contain code related to users in general.
pub mod student;
// ------------------
// This tells Rust: "We have a module named `student`."
// All code related to students (like the Student struct) will be in this module.
pub mod teacher;
// ------------------
// This tells Rust: "We have a module named `teacher`."
// All code related to teachers (like the Teacher struct) will be in this module.
pub mod book;
// ---------------
// This tells Rust: "We have a module named `book`."
// All code related to books (like the Book struct) will be in this module.
pub mod borrow_record;
// ------------------------
// This tells Rust: "We have a module named `borrow_record`."
// All code related to borrowing records (like the BorrowRecord struct) will be in this module.
