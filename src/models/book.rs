pub struct Book {
    // `pub id: u32` 
    // ----------------
    // `id` is a number that uniquely identifies each book.
    // `u32` means it's an unsigned 32-bit integer (just a positive whole number).
    // `pub` means other parts of the program can access this field.

    pub id: u32,

    // `pub title: String`
    // ------------------
    // `title` is the name of the book (like "Harry Potter").
    // `String` is a text type in Rust that can grow or shrink.
    pub title: String,

    // `pub author: String`
    // -------------------
    // `author` is the name of the person who wrote the book.
    pub author: String,

    // `pub isbn: String`
    // -----------------
    // `isbn` is the International Standard Book Number.
    // It's a unique code that identifies the book worldwide.
    pub isbn: String,

    // `pub is_available: bool`
    // -----------------------
    // `is_available` tells us if the book is currently available to borrow.
    // `bool` means it can only be `true` (available) or `false` (borrowed).
    pub is_available: bool,
}
