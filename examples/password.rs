extern crate dialoguer;

use dialoguer::PasswordInput;

fn main() {
    let password = PasswordInput::new("Password")
        .confirm("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();
    println!("Your password is {} characters long", password.len());
}
