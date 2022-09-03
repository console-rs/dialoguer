use std::io;

use crate::theme::{SimpleTheme, TermThemeRenderer, Theme};

use console::Term;
use zeroize::Zeroizing;

/// Renders a password input prompt.
///
/// ## Example usage
///
/// ```rust,no_run
/// # fn test() -> Result<(), Box<std::error::Error>> {
/// use dialoguer::Password;
///
/// let password = Password::new().with_prompt("New Password")
///     .with_confirmation("Confirm password", "Passwords mismatching")
///     .interact()?;
/// println!("Length of the password is: {}", password.len());
/// # Ok(()) } fn main() { test().unwrap(); }
/// ```
pub struct Password<'a> {
    prompt: String,
    report: bool,
    theme: &'a dyn Theme,
    allow_empty_password: bool,
    confirmation_prompt: Option<(String, String)>,
}

impl Default for Password<'static> {
    fn default() -> Password<'static> {
        Self::new()
    }
}

impl Password<'static> {
    /// Creates a password input prompt.
    pub fn new() -> Password<'static> {
        Self::with_theme(&SimpleTheme)
    }
}

impl Password<'_> {
    /// Sets the password input prompt.
    pub fn with_prompt<S: Into<String>>(&mut self, prompt: S) -> &mut Self {
        self.prompt = prompt.into();
        self
    }

    /// Indicates whether to report confirmation after interaction.
    ///
    /// The default is to report.
    pub fn report(&mut self, val: bool) -> &mut Self {
        self.report = val;
        self
    }

    /// Enables confirmation prompting.
    pub fn with_confirmation<A, B>(&mut self, prompt: A, mismatch_err: B) -> &mut Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        self.confirmation_prompt = Some((prompt.into(), mismatch_err.into()));
        self
    }

    /// Allows/Disables empty password.
    ///
    /// By default this setting is set to false (i.e. password is not empty).
    pub fn allow_empty_password(&mut self, allow_empty_password: bool) -> &mut Self {
        self.allow_empty_password = allow_empty_password;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stderr.
    pub fn interact(&self) -> io::Result<String> {
        self.interact_on(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<String> {
        let mut render = TermThemeRenderer::new(term, self.theme);
        render.set_prompts_reset_height(false);

        loop {
            let password = Zeroizing::new(self.prompt_password(&mut render, &self.prompt)?);

            if let Some((ref prompt, ref err)) = self.confirmation_prompt {
                let pw2 = Zeroizing::new(self.prompt_password(&mut render, prompt)?);

                if *password == *pw2 {
                    render.clear()?;
                    if self.report {
                        render.password_prompt_selection(&self.prompt)?;
                    }
                    term.flush()?;
                    return Ok((*password).clone());
                }

                render.error(err)?;
            } else {
                render.clear()?;
                if self.report {
                    render.password_prompt_selection(&self.prompt)?;
                }
                term.flush()?;

                return Ok((*password).clone());
            }
        }
    }

    fn prompt_password(&self, render: &mut TermThemeRenderer, prompt: &str) -> io::Result<String> {
        loop {
            render.password_prompt(prompt)?;
            render.term().flush()?;

            let input = render.term().read_secure_line()?;

            render.add_line();

            if !input.is_empty() || self.allow_empty_password {
                return Ok(input);
            }
        }
    }
}

impl<'a> Password<'a> {
    /// Creates a password input prompt with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            prompt: "".into(),
            report: true,
            theme,
            allow_empty_password: false,
            confirmation_prompt: None,
        }
    }
}
