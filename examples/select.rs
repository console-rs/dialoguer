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

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor, hint it might be on the second page")
        .default(0)
        .max_length(2)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);

    let keys = vec![console::Key::Char('q'),console::Key::Char('s')];

    let selection_with_keys = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor (press q or s to skip)")
        .default(0)
        .items(&selections[..])
        .interact_opt_with_keys(&keys)
            .unwrap();
    
    if let Some(index) = selection_with_keys.index {
        println!("Enjoy your {}!", selections[index]);
    }
    if let Some(key) = selection_with_keys.key {
        println!("You skippng by pressing {:?}!", key);
    }


    
}
