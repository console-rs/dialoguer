extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Datetime};

fn main() {
    let datetime = Datetime::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a time")
        .interact()
        .unwrap();
    println!("Time selected {}", datetime);
}

