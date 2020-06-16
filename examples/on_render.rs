extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Select, MultiSelect, Sort, Password, Input, Confirm};
use dialoguer::theme::SimpleTheme;

fn main() {
    Confirm::with_theme(&ColorfulTheme::default())
        .set_on_render(|b|println!("An render update occurred {}",b))
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap();


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

    let multiselected = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let defaults = &[false, false, true, false];
    let selections = MultiSelect::with_theme(&SimpleTheme)
        .set_on_render(|index| println!("An render update occurred {:?}",index))
        .with_prompt("Pick your food")
        .items(&multiselected[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", multiselected[selection]);
        }
    }

    let list = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let sorted = Sort::with_theme(&ColorfulTheme::default())
        .set_on_render(|index| println!("An render update occurred {:?}",index))
        .with_prompt("Order your foods by preference")
        .items(&list[..])
        .interact()
        .unwrap();

    println!("Your favorite item:");
    println!("  {}", list[sorted[0]]);
    println!("Your least favorite item:");
    println!("  {}", list[sorted[sorted.len() - 1]]);

    let password = Password::with_theme(&ColorfulTheme::default())
        //.set_on_render(|pw| println!("An render update occurred password is {:?}", pw))
        .with_prompt("Password")
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();
    println!("Your password is {} characters long", password.len());

    let _: String = Input::with_theme(&ColorfulTheme::default())
        .set_on_render(|str| println!("An render update occurred {:?}", str))
        .with_prompt("Your name")
        .interact_text()
        .unwrap();
}
