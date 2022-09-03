use dialoguer::{theme::ColorfulTheme, Input};

fn main() {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your name")
        .interact_text()
        .unwrap();

    println!("Hello {name}!");

    let planet: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your planet")
        .default("Earth".to_string())
        .interact_text()
        .unwrap();

    println!("Planet: {planet}");

    let galaxy: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your galaxy")
        .with_initial_text("Milky Way".to_string())
        .interact_text()
        .unwrap();

    println!("Galaxy: {galaxy}");
}
