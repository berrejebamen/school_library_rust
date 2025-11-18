pub struct BorrowRecord {
    // `pub user_id: u32`
    // -----------------
    // This is the unique ID of the user who borrowed the book.
    // `u32` is an unsigned 32-bit number (positive whole number).
    // `pub` makes it accessible from other parts of the program.
    pub user_id: u32,

    // `pub book_id: u32`
    // -----------------
    // This is the unique ID of the book that was borrowed.
    pub book_id: u32,

    // `pub borrow_date: String`
    // ------------------------
    // This is the date when the book was borrowed.
    // We use a `String` to store it as text (like "2025-11-17").
    pub borrow_date: String,

    // `pub return_date: Option<String>`
    // ---------------------------------
    // This is the date when the book was returned.
    // `Option<String>` means it can either have a value (`Some(date)`) 
    // or be empty (`None`) if the book hasn't been returned yet.
    pub return_date: Option<String>,
}
