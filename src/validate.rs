//! Provides validation for text inputs
/// This trait provides functionality for checking if an input is valid.
///
/// If the validate function returns an Ok value, it means that validation has succeeded. If it returns an error, the user is asked to try again, printing the string within the Err.
pub trait Validator {
    fn validate(&self, text: String) -> Result<(), String>;
}