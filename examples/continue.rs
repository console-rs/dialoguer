extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Confirmation, Input};

fn main() {
    if Confirmation::with_theme(&ColorfulTheme)
        .with_text("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
        return;
    }

    let input = Input::with_theme(&ColorfulTheme)
        .with_prompt("Your name")
        .interact()
        .unwrap();
    println!("Hello {}!", input);
}
