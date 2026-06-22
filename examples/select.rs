use console::style;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        println!("Enjoy your {}!", selections[selection]);
    } else {
        println!("You didn't select anything!");
    }

    let theme = ColorfulTheme {
        header: style("-----------------------".to_string()).for_stderr(),
        footer: style("-----------------------".to_string()).for_stderr(),
        ..ColorfulTheme::default()
    };
    let selection = Select::with_theme(&theme)
        .with_prompt("Pick your flavor, hint it might be on the second page")
        .default(0)
        .max_length(2)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}
