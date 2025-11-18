pub trait User {
    // This function must return the user's ID as a u32 number
    fn get_id(&self) -> u32;

    // This function must return the user's name as a string slice (&str)
    fn get_name(&self) -> &str;
}
