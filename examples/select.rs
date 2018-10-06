extern crate dialoguer;

use dialoguer::{ColorfulTheme, Select};

fn main() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Select::new()
        .theme(ColorfulTheme)
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    println!("Enjoy your {}!", selections[selection]);
}
