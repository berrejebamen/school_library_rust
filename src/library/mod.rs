// `pub mod library;`
// ----------------------
// This tells Rust: "We have a module called `library` in this project."
// A module is like a little box where we keep related code together.
// In our case, all library-related stuff 
// (books, students, teachers, borrow records, etc.) will be inside this `library` module.

// `pub` means "public".
// Without `pub`, other parts of the program (like `main.rs`) CANNOT use this module.
// By writing `pub`, we are saying: "Other files in this project can use the stuff inside `library`."

// Rust will look for the code for this module in one of these places:
// 1. A file named `library.rs` in the same folder.
// 2. Or a folder named `library` with a `mod.rs` inside (less common in newer Rust).
pub mod library;


