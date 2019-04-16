extern crate dialoguer;
use dialoguer::{Input, Validator};

struct EmailValidator;

impl Validator for EmailValidator {
    type Err = String;

    fn validate(&self, value: &str) -> Result<(), String> {
        if value.contains('@') {
            Ok(())
        } else {
            Err("not an email address".into())
        }
    }
}

fn main() {
    let email: String = Input::new()
        .with_prompt("Enter an email address")
        .validate_with(EmailValidator)
        .interact().unwrap();
    println!("email: {}", email);
}
