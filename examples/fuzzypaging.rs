use dialoguer::{
    console::{Style, Term},
    theme::ColorfulTheme,
    FuzzySelect,
};

fn main() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
        "Carrots",
        "Peas",
        "Pistacio",
        "Mustard",
        "Cream",
        "Banana",
        "Chocolate",
        "Flakes",
        "Corn",
        "Cake",
        "Tarte",
        "Cheddar",
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
        "Burger",
    ];

    let theme = ColorfulTheme {
        active_item_style: Style::new().for_stderr().on_green().black(),
        ..Default::default()
    };

    let selection = FuzzySelect::with_theme(&theme)
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(sel) = selection {
        println!("Enjoy your {}!", selections[sel]);
    };
}
