extern crate dialoguer;

use dialoguer::Input;

fn main() {
    let mail: String = Input::new()
        .with_prompt("Enter email")
        .validate_with(|input: &str| -> Result<(), &str> {
            if input.contains('@') {
                Ok(())
            } else {
                Err("This is not a mail address")
            }
        })
        .interact()
        .unwrap();

    println!("Email: {}", mail);
}
