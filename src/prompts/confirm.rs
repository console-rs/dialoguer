use std::io;

use crate::theme::{SimpleTheme, TermThemeRenderer, Theme};

use console::Term;

/// Renders a confirm prompt.
///
/// ## Example usage
///
/// ```rust,no_run
/// # fn test() -> Result<(), Box<std::error::Error>> {
/// use dialoguer::Confirm;
///
/// if Confirm::new().with_text("Do you want to continue?").interact()? {
///     println!("Looks like you want to continue");
/// } else {
///     println!("nevermind then :(");
/// }
/// # Ok(()) } fn main() { test().unwrap(); }
/// ```
pub struct Confirm<'a> {
    prompt: String,
    default: bool,
    show_default: bool,
    theme: &'a dyn Theme,
}

impl<'a> Default for Confirm<'a> {
    fn default() -> Confirm<'a> {
        Confirm::new()
    }
}

impl<'a> Confirm<'a> {
    /// Creates a confirm prompt.
    pub fn new() -> Confirm<'static> {
        Confirm::with_theme(&SimpleTheme)
    }

    /// Creates a confirm prompt with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> Confirm<'a> {
        Confirm {
            prompt: "".into(),
            default: true,
            show_default: true,
            theme,
        }
    }

    /// Sets the confirm prompt.
    pub fn with_prompt<S: Into<String>>(&mut self, prompt: S) -> &mut Confirm<'a> {
        self.prompt = prompt.into();
        self
    }

    #[deprecated(note = "Use with_prompt() instead", since = "0.6.0")]
    #[inline]
    pub fn with_text(&mut self, text: &str) -> &mut Confirm<'a> {
        self.with_prompt(text)
    }

    /// Overrides the default.
    pub fn default(&mut self, val: bool) -> &mut Confirm<'a> {
        self.default = val;
        self
    }

    /// Disables or enables the default value display.
    ///
    /// The default is to append `[y/n]` to the prompt to tell the
    /// user which keys to press. This also renders the default choice
    /// in uppercase. The default is selected on enter.
    pub fn show_default(&mut self, val: bool) -> &mut Confirm<'a> {
        self.show_default = val;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stderr.
    pub fn interact(&self) -> io::Result<bool> {
        self.interact_on(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<bool> {
        let mut render = TermThemeRenderer::new(term, self.theme);

        render.confirm_prompt(
            &self.prompt,
            if self.show_default {
                Some(self.default)
            } else {
                None
            },
        )?;

        term.hide_cursor()?;
        term.flush()?;

        loop {
            let input = term.read_char()?;
            let rv = match input {
                'y' | 'Y' => true,
                'n' | 'N' => false,
                '\n' | '\r' => self.default,
                _ => {
                    continue;
                }
            };

            term.clear_line()?;
            render.confirm_prompt_selection(&self.prompt, rv)?;
            term.show_cursor()?;
            term.flush()?;

            return Ok(rv);
        }
    }
}
