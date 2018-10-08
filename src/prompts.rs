use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

use theme::{get_default_theme, TermThemeRenderer, Theme};

use console::Term;

/// Renders a simple confirmation prompt.
///
/// ## Example usage
///
/// ```rust,no_run
/// # fn test() -> Result<(), Box<std::error::Error>> {
/// use dialoguer::Confirmation;
///
/// if Confirmation::new().with_text("Do you want to continue?").interact()? {
///     println!("Looks like you want to continue");
/// } else {
///     println!("nevermind then :(");
/// }
/// # Ok(()) } fn main() { test().unwrap(); }
/// ```
pub struct Confirmation<'a> {
    text: String,
    default: bool,
    show_default: bool,
    theme: &'a Theme,
}

/// Renders a simple input prompt.
///
/// ## Example usage
///
/// ```rust,no_run
/// # fn test() -> Result<(), Box<std::error::Error>> {
/// use dialoguer::Input;
///
/// let name = Input::new().with_prompt("Your name").interact()?;
/// println!("Name: {}", name);
/// # Ok(()) } fn main() { test().unwrap(); }
/// ```
pub struct Input<'a, T> {
    prompt: String,
    default: Option<T>,
    show_default: bool,
    theme: &'a Theme,
}

/// Renders a password input prompt.
///
/// ## Example usage
///
/// ```rust,no_run
/// # fn test() -> Result<(), Box<std::error::Error>> {
/// use dialoguer::PasswordInput;
///
/// let password = PasswordInput::new().with_prompt("New Password")
///     .with_confirmation("Confirm password", "Passwords mismatching")
///     .interact()?;
/// println!("Length of the password is: {}", password.len());
/// # Ok(()) } fn main() { test().unwrap(); }
/// ```
pub struct PasswordInput<'a> {
    prompt: String,
    theme: &'a Theme,
    confirmation_prompt: Option<(String, String)>,
}

impl<'a> Confirmation<'a> {
    /// Creates the prompt with a specific text.
    pub fn new() -> Confirmation<'static> {
        Confirmation::with_theme(get_default_theme())
    }

    /// Sets a theme other than the default one.
    pub fn with_theme(theme: &'a Theme) -> Confirmation<'a> {
        Confirmation {
            text: "".into(),
            default: true,
            show_default: true,
            theme: theme,
        }
    }

    /// Sets the confirmation text.
    pub fn with_text(&mut self, text: &str) -> &mut Confirmation<'a> {
        self.text = text.into();
        self
    }

    /// Overrides the default.
    pub fn default(&mut self, val: bool) -> &mut Confirmation<'a> {
        self.default = val;
        self
    }

    /// Disables or enables the default value display.
    ///
    /// The default is to append `[y/n]` to the prompt to tell the
    /// user which keys to press.  This also renders the default choice
    /// in uppercase.  The default is selected on enter.
    pub fn show_default(&mut self, val: bool) -> &mut Confirmation<'a> {
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

        render.confirmation_prompt(
            &self.text,
            if self.show_default {
                Some(self.default)
            } else {
                None
            },
        )?;
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
            render.confirmation_prompt_selection(&self.text, rv)?;
            return Ok(rv);
        }
    }
}

impl<'a, T> Input<'a, T>
where
    T: Clone + FromStr + Display,
    T::Err: Display + Debug,
{
    /// Creates a new input prompt.
    pub fn new() -> Input<'static, T> {
        Input::with_theme(get_default_theme())
    }

    /// Creats an input with a specific theme.
    pub fn with_theme(theme: &'a Theme) -> Input<'a, T> {
        Input {
            prompt: "".into(),
            default: None,
            show_default: true,
            theme: theme,
        }
    }

    /// Sets the input prompt.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut Input<'a, T> {
        self.prompt = prompt.into();
        self
    }

    /// Sets a default.
    ///
    /// Out of the box the prompt does not have a default and will continue
    /// to display until the user hit enter.  If a default is set the user
    /// can instead accept the default with enter.
    pub fn default(&mut self, value: T) -> &mut Input<'a, T> {
        self.default = Some(value);
        self
    }

    /// Disables or enables the default value display.
    ///
    /// The default is to append `[default]` to the prompt to tell the
    /// user that a default is acceptable.
    pub fn show_default(&mut self, val: bool) -> &mut Input<'a, T> {
        self.show_default = val;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stderr.
    pub fn interact(&self) -> io::Result<T> {
        self.interact_on(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<T> {
        let mut render = TermThemeRenderer::new(term, self.theme);

        loop {
            let default_string = self.default.as_ref().map(|x| x.to_string());
            render.input_prompt(
                &self.prompt,
                if self.show_default {
                    default_string.as_ref().map(|x| x.as_str())
                } else {
                    None
                },
            )?;
            let input = term.read_line()?;
            render.add_line();
            if input.is_empty() {
                render.clear()?;
                if let Some(ref default) = self.default {
                    render.single_prompt_selection(&self.prompt, &default.to_string())?;
                    return Ok(default.clone());
                } else {
                    continue;
                }
            }
            render.clear()?;
            match input.parse::<T>() {
                Ok(value) => {
                    render.single_prompt_selection(&self.prompt, &input)?;
                    return Ok(value);
                }
                Err(err) => {
                    render.error(&err.to_string())?;
                    continue;
                }
            }
        }
    }
}

impl<'a> PasswordInput<'a> {
    /// Creates a new input prompt.
    pub fn new() -> PasswordInput<'static> {
        PasswordInput::with_theme(get_default_theme())
    }

    /// Creates the password input with a specific theme.
    pub fn with_theme(theme: &'a Theme) -> PasswordInput<'a> {
        PasswordInput {
            prompt: "".into(),
            theme: theme,
            confirmation_prompt: None,
        }
    }

    /// Sets the prompt.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut PasswordInput<'a> {
        self.prompt = prompt.into();
        self
    }

    /// Enables confirmation prompting.
    pub fn with_confirmation(
        &mut self,
        prompt: &str,
        mismatch_err: &str,
    ) -> &mut PasswordInput<'a> {
        self.confirmation_prompt = Some((prompt.into(), mismatch_err.into()));
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
            let password = self.prompt_password(&mut render, &self.prompt)?;
            if let Some((ref prompt, ref err)) = self.confirmation_prompt {
                let pw2 = self.prompt_password(&mut render, &prompt)?;
                if password == pw2 {
                    render.clear()?;
                    render.password_prompt_selection(&self.prompt)?;
                    return Ok(password);
                }
                render.error(err)?;
            } else {
                render.clear()?;
                render.password_prompt_selection(&self.prompt)?;
                return Ok(password);
            }
        }
    }

    fn prompt_password(&self, render: &mut TermThemeRenderer, prompt: &str) -> io::Result<String> {
        loop {
            render.password_prompt(prompt)?;
            let input = render.term().read_secure_line()?;
            render.add_line();
            if !input.is_empty() {
                return Ok(input);
            }
        }
    }
}
