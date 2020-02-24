extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Autocomplete};

fn main() {
    let items = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
        "Vanilla Cupcake 1",
        "Vanilla Cupcake 2",
        "Vanilla Cupcake 3",
        "Vanilla Cupcake 4",
        "Vanilla Cupcake 5",
        "Vanilla Cupcake 6",
        "Vanilla Cupcake 7",
        "Vanilla Cupcake 8",
        "Vanilla Cupcake 9",
        "Vanilla Cupcake 10",
        "Vanilla Cupcake 11",
        "Vanilla Cupcake 12",
        "Vanilla Cupcake 13",
    ];

    let selection = Autocomplete::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .items(&items[..])
        .paged(true)
        .interact_opt()
        .unwrap();
    if let Some(sel) = selection {
        println!("Enjoy your {}!", items[sel]);
    } else {
        println!("Quitted.")
    }
}
