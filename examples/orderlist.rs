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
        .with_prompt("Order your foods by preference")
        .items(&list[..])
        .interact()
        .unwrap();

    println!("Your favorite item:");
    println!("  {}", list[order_list[0]]);
    println!("Your least favorite item:");
    println!("  {}", list[order_list[order_list.len() - 1]]);
}
