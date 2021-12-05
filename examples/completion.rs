#![cfg(feature = "completion")]
use dialoguer::{theme::ColorfulTheme, Completion, Input};

fn main() -> Result<(), std::io::Error> {
    println!("Use the Right arrow or Tab to complete your command");
    let completion = MyCompletion::default();
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("dialoguer")
        .completion_with(&completion)
        .interact_text()?;
    Ok(())
}

struct MyCompletion {
    options: Vec<String>,
}

impl Default for MyCompletion {
    fn default() -> Self {
        MyCompletion {
            options: vec![
                "orange".to_string(),
                "apple".to_string(),
                "banana".to_string(),
            ],
        }
    }
}

impl Completion for MyCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let s = input.to_string();
        let ss: Vec<&String> = self.options.iter().filter(|x| s == x[..s.len()]).collect();
        if ss.len() == 1 {
            Some(ss[0].to_string())
        } else {
            None
        }
    }
}
