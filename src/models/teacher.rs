use super::user::User;
pub struct Teacher {
    // `pub id: u32`
    // ----------------
    // Each teacher has a unique ID number.
    // `u32` is an unsigned 32-bit number (only positive numbers).
    // `pub` makes this field accessible from other parts of the program.
    pub id: u32,

    // `pub name: String`
    // -----------------
    // Each teacher has a name stored as a String (text).
    pub name: String,
}

// Implement the `User` trait for `Teacher`
// ----------------------------------------
// This means that a `Teacher` can do the actions a `User` must be able to do.
impl User for Teacher {
    // Implement the `get_id` function from the `User` trait
    fn get_id(&self) -> u32 {
        // Return this teacher's ID number
        self.id
    }

    // Implement the `get_name` function from the `User` trait
    fn get_name(&self) -> &str {
        // Return a reference to this teacher's name
        &self.name
    }
}

