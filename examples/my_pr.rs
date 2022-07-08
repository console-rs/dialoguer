use dialoguer::{theme::ColorfulTheme, Input};
use console::Term;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::buffered_stdout();
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your name")
        .with_initial_text("abcdef")
        .report(true)
        .interact_text_on(&term)?;
    Ok(())
}
