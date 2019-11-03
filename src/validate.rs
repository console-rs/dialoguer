//! Provides validation for text inputs
use std::fmt::{Debug, Display};
pub trait Validator {
    type Err: Debug + Display;

    /// Invoked with the value to validate.
    ///
    /// If this produces `Ok(())` then the value is used and parsed, if
    /// an error is returned validation fails with that error.
    fn validate(&self, text: &str) -> Result<(), Self::Err>;
}

impl<T: Fn(&str) -> Result<(), E>, E: Debug + Display> Validator for T {
    type Err = E;

    fn validate(&self, text: &str) -> Result<(), Self::Err> {
        self(text)
    }
}
