extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .set_on_render(|index| println!("An render update occurred {}",index))
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    println!("Enjoy your {}!", selections[selection]);
}
