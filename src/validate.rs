//! Provides validation for text inputs
#[cfg(feature = "validation")]
use regex::Regex;
use std::ops::Fn;

/// This trait provides functionality for checking if an input is valid.
///
/// If the validate function returns an Ok value, it means that validation has succeeded. If it returns an error, the user is asked to try again, printing the string within the Err.
pub trait Validator {
    fn validate(&self, text: String) -> Result<(), String>;
}
/// A set of prebuilt validators
///
/// These are structs that come pre-built with an implementation of Validator, so that you can easily validate inputs
#[cfg(feature = "validation")]
pub mod prebuilt {
    use super::*;
    /// Validates phone numbers
    ///
    /// This should support most common representations of phone numbers. The regexes used come from [this SO answer](https://stackoverflow.com/a/16702965).
    pub struct PhoneNumber {
        require_cc: bool,
        regex: Regex,
        code_regex: Regex,
    }
    impl PhoneNumber {
        pub fn default() -> PhoneNumber {
            PhoneNumber {
                require_cc: false,
                regex: Regex::new(r"^\s*(?:\+?(\d{1,3}))?[-. (]*(\d{3})[-. )]*(\d{3})[-. ]*(\d{4})(?: *x(\d+))?\s*$").unwrap(),
                code_regex: Regex::new(r"^\s*(?:\+?(\d{1,3}))[-. (]*(\d{3})[-. )]*(\d{3})[-. ]*(\d{4})(?: *x(\d+))?\s*$").unwrap(),
            }
        }
        /// Sets whether or not a country code prefixing the number is required
        pub fn country_code(&mut self, val: bool) -> &mut PhoneNumber {
            self.require_cc = val;
            self
        }
    }
    impl Validator for PhoneNumber {
        fn validate(&self, text: String) -> Result<(), String> {
            if match self.require_cc {
                true => &self.code_regex,
                false => &self.regex,
            }
            .is_match(&text)
            {
                Ok(())
            } else {
                Err("Please enter a valid phone number".to_string())
            }
        }
    }
    /// Validates email addresses
    ///
    /// The regex used here isn't the most sophisticated, but should work just fine for almost all emails. I borrowed it from [this site](https://www.regular-expressions.info/index.html).
    pub struct EmailAddress {
        regex: Regex,
    }
    impl EmailAddress {
        pub fn default() -> EmailAddress {
            EmailAddress {
                regex: Regex::new(r"(?i:[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,})").unwrap(),
            }
        }
    }
    impl Validator for EmailAddress {
        fn validate(&self, text: String) -> Result<(), String> {
            if self.regex.is_match(&text) {
                Ok(())
            } else {
                Err("Please enter a valid email address".to_string())
            }
        }
    }
    /// A custom validator
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # fn test() {
    /// use dialoguer::{ValidatedInput, validate::prebuilt::EasyCustomValidator};
    /// let mut short : String = ValidatedInput::new(EasyCustomValidator::from_func(
    /// |text| -> Result<(), String> {
    ///     if text.len() > 5 {
    ///     return Err("Please enter a longer string".to_string());
    /// }
    /// Ok(())
    /// })).interact().unwrap();
    ///
    /// # }
    /// ```
    pub struct EasyCustomValidator<F> {
        function: F,
    }
    impl<F> EasyCustomValidator<F>
    where
        F: Fn(String) -> Result<(), String>,
    {
        pub fn from_func(func: F) -> EasyCustomValidator<F> {
            EasyCustomValidator { function: func }
        }
    }
    impl<F> Validator for EasyCustomValidator<F>
    where
        F: Fn(String) -> Result<(), String>,
    {
        fn validate(&self, text: String) -> Result<(), String> {
            (self.function)(text)
        }
    }
}
