use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
        "Foo",
        "Bar",
        "Baz",
        "Mustard",
        "Cream",
        "Banana",
        "Chocolate",
        "Flakes",
        "Corn",
        "Cake",
        "Tarte",
        "Clear glaze",
        "Vanilla",
        "Hazelnut",
        "Flour",
        "Sugar",
        "Salt",
        "Potato",
        "French Fries",
        "Pizza",
        "Mousse au chocolat",
        "Brown sugar",
        "Blueberry",
        "Blanc",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .paged(true)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);

    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Optionally pick your flavor")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact_opt()
    //     .unwrap();

    // if let Some(selection) = selection {
    //     println!("Enjoy your {}!", selections[selection]);
    // } else {
    //     println!("You didn't select anything!");
    // }
}
