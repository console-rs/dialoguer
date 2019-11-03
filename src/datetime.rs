use std::io;

use theme::{get_default_theme, TermThemeRenderer, Theme};
use chrono::{DateTime, Datelike, Timelike, Utc};
use console::{Key, Term};

// TODO: Add fields to set default time.
pub struct Datetime<'a> {
    prompt: Option<String>,
    default: Option<String>,
    theme: &'a Theme,
    weekday: bool,
}

impl <'a> Datetime<'a> {
    pub fn new() -> Datetime<'static> {
        Datetime::with_theme(get_default_theme())
    }

    /// Creates a datetime with a specific theme.
    pub fn with_theme(theme: &'a Theme) -> Datetime<'a> {
        Datetime {
            prompt: None,
            default: None,
            theme,
            weekday: true,
        }
    }
    /// Sets the input prompt.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut Datetime<'a> {
        self.prompt = Some(prompt.into());
        self
    }
    /// Sets default time to start with.
    pub fn default(&mut self, datetime: &str) -> &mut Datetime<'a> {
        self.default = Some(datetime.into());
        self
    }
    /// Sets weekday time to start with.
    pub fn weekday(&mut self, val: bool) -> &mut Datetime<'a> {
        self.weekday = val;
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
        let now = Utc::now() 
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();

        let now = format!(
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}-00:00",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
        );

        let mut date_val = match &self.default {
            Some(datetime) => {
                DateTime::parse_from_rfc3339(datetime).expect("date format must match rfc3339")
            },
            None => {
                DateTime::parse_from_rfc3339(&now).expect("date format must match rfc3339")
            }
        };
        let mut render = TermThemeRenderer::new(term, self.theme);
        loop {
            let date_str = format!(
                "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                date_val.year(),
                date_val.month(),
                date_val.day(),
                date_val.hour(),
                date_val.minute(),
                date_val.second(),
            );

            // Add weekday if specified.
            let date_str = match &self.weekday {
                true => format!("{}, {:?}", date_str, date_val.weekday()),
                false => date_str,
            };

            render.datetime(&self.prompt, &date_str);
            match term.read_key()? {
                Key::Enter => {
                    return Ok(date_str.to_owned());
                },
                _ => {}
            }
            render.clear()?;
        }
    }
}

