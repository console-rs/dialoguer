extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Checkboxes};

fn main() {
    let checkboxes = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let selections = Checkboxes::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your food")
        .items(&checkboxes[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", checkboxes[selection]);
        }
    }
}
