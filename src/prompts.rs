use std::io;

use console::Term;

pub struct Confirmation {
    text: String,
    default: bool,
    show_default: bool,
    line_input: bool,
    clear: bool,
}

impl Confirmation {
    pub fn new(text: &str) -> Confirmation {
        Confirmation {
            text: text.into(),
            default: true,
            show_default: true,
            line_input: false,
            clear: true,
        }
    }

    pub fn use_line_input(&mut self, val: bool) -> &mut Confirmation {
        self.line_input = val;
        self
    }

    pub fn clear(&mut self, val: bool) -> &mut Confirmation {
        self.clear = val;
        self
    }

    pub fn interact(&self) -> io::Result<bool> {
        self.interact_on(&Term::stdout())
    }

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
                if self.clear {
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
                if self.clear {
                    term.clear_last_lines(1)?;
                }
                return Ok(rv);
            }
        }
    }
}
