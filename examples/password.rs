extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, PasswordInput};

fn main() {
    let password = PasswordInput::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();
    println!("Your password is {} characters long", password.len());
}
