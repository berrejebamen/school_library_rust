use super::user::User;
pub struct Student {
    // `pub id: u32` 
    // ----------------
    // Each student has a unique ID number.
    // `u32` is an unsigned 32-bit number (positive whole number).
    // `pub` means other modules can access this field.
    pub id: u32,

    // `pub name: String`
    // -----------------
    // Each student has a name stored as a String (text).
    pub name: String,
}

// Implement the `User` trait for `Student`
// ----------------------------------------
// This means that a `Student` must provide the functions defined in the `User` trait.
impl User for Student {
    // Implement the `get_id` function from the `User` trait
    fn get_id(&self) -> u32 {
        // Return this student's ID
        self.id
    }

    // Implement the `get_name` function from the `User` trait
    fn get_name(&self) -> &str {
        // Return a reference to this student's name
        &self.name
    }
}
