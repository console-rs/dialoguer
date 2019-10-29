use theme::{get_default_theme, Theme};

pub struct Datetime<'a> {
    prompt: String,
    default: String,
    clear: bool,
    theme: &'a Theme,
}

impl <'a> Datetime<'a> {
    pub fn new() -> Datetime<'static> {
        Datetime::with_theme(get_default_theme())
    }
}

