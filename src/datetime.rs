use std::io;

use theme::{get_default_theme, TermThemeRenderer, Theme};
use chrono::{Timelike, Utc};
use console::{Key, Term};

// TODO: Add fields to set default time.
pub struct Datetime<'a> {
    prompt: Option<String>,
    theme: &'a Theme,
}

impl <'a> Datetime<'a> {
    pub fn new() -> Datetime<'static> {
        Datetime::with_theme(get_default_theme())
    }

    /// Creates a datetime with a specific theme.
    pub fn with_theme(theme: &'a Theme) -> Datetime<'a> {
        Datetime {
            prompt: None,
            theme,
        }
    }
    /// Sets the input prompt.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut Datetime<'a> {
        self.prompt = Some(prompt.into());
        self
    }
    /// Enables user interaction and returns the result.
    ///
    /// The dialog is rendered on stderr.
    pub fn interact(&self) -> io::Result<String> {
        self.interact_on(&Term::stderr())
    }
    /// Like `interact` but allows a specific terminal to be set.
    // TODO: rework this to handle logic for changing date.
    fn interact_on(&self, term: &Term) -> io::Result<String> {
        let now = Utc::now();
        let (is_pm, hour) = now.hour12();
        let date_val = format!(
            "{:02}:{:02}:{:02} {}",
            hour,
            now.minute(),
            now.second(),
            if is_pm { "PM" } else { "AM" }
        );
        let mut render = TermThemeRenderer::new(term, self.theme);
        loop {
            render.datetime(&self.prompt, &date_val);
            match term.read_key()? {
                Key::Enter => {
                    return Ok(date_val.to_owned());
                },
                _ => {}
            }
            render.clear()?;
        }
    }
}

