use std::io;
use std::fmt::Write;

use console::Term;

/// Renders a simple confirmation prompt.
pub struct Confirmation {
    text: String,
    default: bool,
    show_default: bool,
    line_input: bool,
    clear: Option<bool>,
}

/// Renders a simple input prompt.
pub struct Input {
    text: String,
    default: Option<String>,
    show_default: bool,
    clear: bool,
}

impl Confirmation {
    /// Creates the prompt with a specific text.
    pub fn new(text: &str) -> Confirmation {
        Confirmation {
            text: text.into(),
            default: true,
            show_default: true,
            line_input: false,
            clear: None,
        }
    }

    /// Enables or disables the line input mode.
    ///
    /// The default is to read a single character and to continue the
    /// moment the key was pressed.  In the line input mode multiple
    /// inputs are allowed and the return key confirms the selection.
    /// In that case if the input is incorrect the prompt is rendered
    /// again.
    pub fn use_line_input(&mut self, val: bool) -> &mut Confirmation {
        self.line_input = val;
        self
    }

    /// Sets the clear behavior of the prompt.
    ///
    /// The default is to clear the prompt if line input is disabled
    /// and to clear otherwise.
    pub fn clear(&mut self, val: bool) -> &mut Confirmation {
        self.clear = Some(val);
        self
    }

    /// Overrides the default.
    pub fn default(&mut self, val: bool) -> &mut Confirmation {
        self.default = val;
        self
    }

    /// Disables or enables the default value display.
    ///
    /// The default is to append `[y/n]` to the prompt to tell the
    /// user which keys to press.  This also renders the default choice
    /// in uppercase.  The default is selected on enter.
    pub fn show_default(&mut self, val: bool) -> &mut Confirmation {
        self.show_default = val;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stdout.
    pub fn interact(&self) -> io::Result<bool> {
        self.interact_on(&Term::stdout())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<bool> {
        let prompt = format!("{}{} ", &self.text, if self.show_default {
            if self.default { " [Y/n]" } else { " [y/N]" }
        } else {
            ""
        });

        if !self.line_input {
            term.write_str(&prompt)?;
            loop {
                let input = term.read_char()?;
                let rv = match input {
                    'y' | 'Y' => true,
                    'n' | 'N' => false,
                    '\n' | '\r' => self.default,
                    _ => { continue; }
                };
                if self.clear.unwrap_or(true) {
                    term.clear_line()?;
                } else {
                    term.write_line("")?;
                }
                return Ok(rv);
            }
        } else {
            loop {
                term.write_str(&prompt)?;
                let input = term.read_line()?;
                let rv = match input.trim() {
                    "y" | "Y" => true,
                    "n" | "N" => false,
                    "\n" | "\r" => self.default,
                    _ => { continue; }
                };
                if self.clear.unwrap_or(false) {
                    term.clear_last_lines(1)?;
                }
                return Ok(rv);
            }
        }
    }
}

impl Input {
    /// Creates a new input prompt.
    pub fn new(text: &str) -> Input {
        Input {
            text: text.into(),
            default: None,
            show_default: true,
            clear: false,
        }
    }

    /// Sets the clear behavior of the prompt.
    ///
    /// The default is not to clear.
    pub fn clear(&mut self, val: bool) -> &mut Input {
        self.clear = val;
        self
    }

    /// Sets a default.
    ///
    /// Out of the box the prompt does not have a default and will continue
    /// to display until the user hit enter.  If a default is set the user
    /// can instead accept the default with enter.
    pub fn default(&mut self, s: &str) -> &mut Input {
        self.default = Some(s.into());
        self
    }

    /// Disables or enables the default value display.
    ///
    /// The default is to append `[default]` to the prompt to tell the
    /// user that a default is acceptable.
    pub fn show_default(&mut self, val: bool) -> &mut Input {
        self.show_default = val;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stdout.
    pub fn interact(&self) -> io::Result<String> {
        self.interact_on(&Term::stdout())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<String> {
        let mut prompt = self.text.clone();
        if self.show_default && self.default.is_some() {
            write!(&mut prompt, " [{}]", self.default.as_ref().unwrap()).ok();
        }
        prompt.push_str(": ");

        loop {
            term.write_str(&prompt)?;
            let input = term.read_line()?;
            if input.is_empty() {
                if let Some(ref d) = self.default {
                    return Ok(d.to_string());
                } else {
                    continue;
                }
            }
            if self.clear {
                term.clear_last_lines(1)?;
            }
            return Ok(input);
        }
    }
}
