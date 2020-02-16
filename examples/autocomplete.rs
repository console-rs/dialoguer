extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Autocomplete};

fn main() {
    let items = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Autocomplete::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .items(&items[..])
        .interact()
        .unwrap();
    println!("Enjoy your {}!", items[selection]);
}
