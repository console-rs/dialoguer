use dialoguer::{theme::ColorfulTheme, Input};
use regex::Regex;

fn main() {
    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your email")
        .map(|mut input: String| {
            if !input.contains('@') {
                input += "@example.com";
            }
            input
        })
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                let address_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
                if address_regex.is_match(input) || force.as_ref().map_or(false, |old| old == input)
                {
                    Ok(())
                } else {
                    force = Some(input.clone());
                    Err("This is not a mail address; type the same value again to force use")
                }
            }
        })
        .interact_text()
        .unwrap();

    println!("Email: {}", mail);
}
