use dialoguer::{theme::ColorfulTheme, Password};

fn main() {
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();

    println!("Your password is {} characters long", password.len());
}
