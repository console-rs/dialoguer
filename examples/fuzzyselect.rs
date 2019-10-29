extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};

fn main() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .show_match(true)
        .interact()
        .unwrap();
    println!("Enjoy your {}!", selection);
}

