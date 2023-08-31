use dialoguer::{theme::ColorfulTheme, Input};
use std::{collections::VecDeque, process};

fn main() {
    println!("Use 'exit' to quit the prompt");
    println!("Use the Up/Down arrows to scroll through history");
    println!();

    let mut history = VecDeque::new();

    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("dialoguer")
            .history_with(&mut history)
            .interact_text()
        {
            if cmd == "exit" {
                process::exit(0);
            }
            println!("Entered {}", cmd);
        }
    }
}
