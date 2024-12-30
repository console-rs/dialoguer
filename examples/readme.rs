//! The purpose of this example is to provide simple examples of how to use each of the dialoguer
//! prompts.

use std::{env::args, thread, time::Duration};

#[cfg(feature = "fuzzy-select")]
use dialoguer::FuzzySelect;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Password, Select, Sort};

fn main() -> dialoguer::Result<()> {
    match args().nth(1) {
        None => println!("No argument provided"),
        Some(arg) => run(arg)?,
    }
    Ok(())
}

fn run(arg: String) -> Result<(), dialoguer::Error> {
    match arg.as_str() {
        "confirm" => confirm()?,
        "confirm-with-default" => confirm_with_default()?,
        "input" => input()?,
        "password" => password()?,
        "editor" => editor()?,
        "select" => select()?,
        "multi-select" => multi_select()?,
        #[cfg(feature = "fuzzy-select")]
        "fuzzy-select" => fuzzy_select()?,
        "sort" => sort()?,
        _ => println!("Invalid argument"),
    }
    thread::sleep(Duration::from_secs(3)); // give the VHS tape time to capture the effect
    Ok(())
}

fn confirm() -> dialoguer::Result<()> {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        println!("Looks like you want to continue");
    }
    Ok(())
}

fn confirm_with_default() -> dialoguer::Result<()> {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .default(true)
        .interact()?
    {
        println!("Looks like you want to continue");
    }
    Ok(())
}

fn input() -> dialoguer::Result<()> {
    let name: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your name?")
        .interact()?;
    println!("Hello, {name}");
    Ok(())
}

fn password() -> dialoguer::Result<()> {
    let password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your password")
        .interact()?;
    println!("Your password is: {password}");
    Ok(())
}

fn editor() -> dialoguer::Result<()> {
    match dialoguer::Editor::new().edit("Some content")? {
        Some(content) => println!("Content: {content:?}"),
        None => println!("File was not saved"),
    }
    Ok(())
}

fn select() -> dialoguer::Result<()> {
    let items = vec!["Apple", "Banana", "Cherry"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your favourite fruit?")
        .items(&items)
        .interact()?;
    println!("You picked: {selection}", selection = items[selection]);
    Ok(())
}

#[cfg(feature = "fuzzy-select")]
fn fuzzy_select() -> dialoguer::Result<()> {
    let items = vec!["Apple", "Banana", "Cherry"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your favourite fruit?")
        .items(&items)
        .interact()?;
    println!("You picked: {selection}", selection = items[selection]);
    Ok(())
}

fn multi_select() -> dialoguer::Result<()> {
    let items = vec!["Apple", "Banana", "Cherry"];
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What are your favourite fruits?")
        .items(&items)
        .interact()?;
    let selected_items: Vec<_> = selection.iter().map(|i| items[*i]).collect();
    println!("You picked: {selected_items:?}");
    Ok(())
}

fn sort() -> dialoguer::Result<()> {
    let items = vec!["Apple", "Banana", "Cherry"];
    let selection = Sort::with_theme(&ColorfulTheme::default())
        .with_prompt("Sort the fruits")
        .items(&items)
        .interact()?;
    let sorted_items: Vec<_> = selection.iter().map(|i| items[*i]).collect();
    println!("You sorted: {sorted_items:?}");
    Ok(())
}
