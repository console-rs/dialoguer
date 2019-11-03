use std::io;

use theme::{get_default_theme, TermThemeRenderer, Theme};
use chrono::{DateTime, Datelike, Timelike, Utc};
use console::{Key, Term, style};

pub enum DateType {
    Date,
    Time,
    DateTime,
}

pub struct DateTimeSelect<'a> {
    prompt: Option<String>,
    default: Option<String>,
    theme: &'a Theme,
    weekday: bool,
    date_type: DateType,
}

impl <'a> DateTimeSelect<'a> {
    pub fn new() -> DateTimeSelect<'static> {
        DateTimeSelect::with_theme(get_default_theme())
    }

    /// Creates a datetime with a specific theme.
    pub fn with_theme(theme: &'a Theme) -> DateTimeSelect<'a> {
        DateTimeSelect {
            prompt: None,
            default: None,
            theme,
            weekday: true,
            date_type: DateType::DateTime,
        }
    }
    /// Sets the input prompt.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut DateTimeSelect<'a> {
        self.prompt = Some(prompt.into());
        self
    }
    /// Sets default time to start with.
    pub fn default(&mut self, datetime: &str) -> &mut DateTimeSelect<'a> {
        self.default = Some(datetime.into());
        self
    }
    /// Sets weekday time to start with.
    pub fn weekday(&mut self, val: bool) -> &mut DateTimeSelect<'a> {
        self.weekday = val;
        self
    }
    /// Sets date selector to date, time, or datetime format.
    pub fn date_type(&mut self, val: &str) -> &mut DateTimeSelect<'a> {
        self.date_type = match val {
            "date" => DateType::Date,
            "time" => DateType::Time,
            "datetime" => DateType::DateTime,
            _ => panic!("Must select from \"date\", \"time\", or \"datetime\" values for \"date_type\" method!")
        };
        self
    }
    /// Enables user interaction and returns the result.
    ///
    /// The dialog is rendered on stderr.
    pub fn interact(&self) -> io::Result<String> {
        self.interact_on(&Term::stderr())
    }
    /// Like `interact` but allows a specific terminal to be set.
    fn interact_on(&self, term: &Term) -> io::Result<String> {
        // Used as default if override not sent.
        let now = Utc::now() 
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();

        let mut date_val = match &self.default {
            Some(datetime) => {
                DateTime::parse_from_rfc3339(datetime).expect("date format must match rfc3339")
            },
            None => {
                DateTime::parse_from_rfc3339(&now.to_rfc3339()).expect("date format must match rfc3339")
            }
        };
        let mut render = TermThemeRenderer::new(term, self.theme);

        let mut pos = 0;
        let max_pos = match &self.date_type {
            DateType::Date => 2,
            DateType::Time => 2,
            DateType::DateTime => 5,
        };

        loop {
            let date_str = match &self.date_type {
                DateType::Date => {
                    format!(
                        "{}-{:02}-{:02}",
                        if pos == 0 { style(date_val.year()).bold() } else { style(date_val.year()) },
                        if pos == 1 { style(date_val.month()).bold() } else { style(date_val.month()) },
                        if pos == 2 { style(date_val.day()).bold() } else { style(date_val.day()) },
                    )
                },
                DateType::Time => {
                    format!(
                        "{:02}:{:02}:{:02}",
                        if pos == 0 { style(date_val.hour()).bold() } else { style(date_val.hour()) },
                        if pos == 1 { style(date_val.minute()).bold() } else { style(date_val.minute()) },
                        if pos == 2 { style(date_val.second()).bold() } else { style(date_val.second()) },
                    )
                },
                DateType::DateTime => {
                    format!(
                        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                        if pos == 0 { style(date_val.year()).bold() } else { style(date_val.year()).dim() },
                        if pos == 1 { style(date_val.month()).bold() } else { style(date_val.month()).dim() },
                        if pos == 2 { style(date_val.day()).bold() } else { style(date_val.day()).dim() },
                        if pos == 3 { style(date_val.hour()).bold() } else { style(date_val.hour()).dim() },
                        if pos == 4 { style(date_val.minute()).bold() } else { style(date_val.minute()).dim() },
                        if pos == 5 { style(date_val.second()).bold() } else { style(date_val.second()).dim() },
                    )
                },
            };

            // Add weekday if specified.
            let date_str = match &self.weekday {
                true => format!("{}, {:?}", date_str, date_val.weekday()),
                false => date_str,
            };

            render.datetime(&self.prompt, &date_str)?;
            match term.read_key()? {
                Key::Enter => {
                    return Ok(date_str.to_owned());
                },
                Key::ArrowRight | Key::Char('l') => {
                    pos = if pos == max_pos {
                        0
                    } else {
                        pos + 1
                    };
                },
                Key::ArrowLeft | Key::Char('h') => {
                    pos = if pos == 0 {
                        max_pos
                    } else {
                        pos - 1
                    };
                },
                // TODO: Add cases for changing date_val.
                _ => {}
            }
            render.clear()?;
        }
    }
}

