extern crate dialoguer;

use dialoguer::Select;

fn main() {
    let selection = Select::new()
        .item("Ice Cream")
        .item("Vanilla Cupcake")
        .item("Chocolate Muffin")
        .item("A Pile of sweet, sweet mustard")
        .interact().unwrap();
    println!("Selected {}", selection);
}
