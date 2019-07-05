extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, OrderList};

fn main() {
    let list = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let order_list = OrderList::with_theme(&ColorfulTheme::default())
        .with_prompt("Order your foods")
        .items(&list[..])
        .interact()
        .unwrap();

    println!("You ordered these things:");
    for item in order_list {
        println!("  {}", list[item]);  
    }
}
