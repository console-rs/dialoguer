use std::io;
use std::ops::Rem;

use console::{Key, Term};

/// Renders a selection menu.
pub struct Select {
    default: usize,
    items: Vec<String>,
    clear: bool,
}

impl Select {
    /// Creates the prompt with a specific text.
    pub fn new() -> Select {
        Select {
            default: !0,
            items: vec![],
            clear: true,
        }
    }

    /// Sets the clear behavior of the menu.
    ///
    /// The default is to clear the menu.
    pub fn clear(&mut self, val: bool) -> &mut Select {
        self.clear = val;
        self
    }

    /// Sets a default for the menu
    pub fn default(&mut self, val: usize) -> &mut Select {
        self.default = val;
        self
    }

    /// Add a single item to the selector.
    pub fn item(&mut self, item: &str) -> &mut Select {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the selector.
    pub fn items(&mut self, items: &[&str]) -> &mut Select {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stdout.
    pub fn interact(&self) -> io::Result<usize> {
        self.interact_on(&Term::stdout())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<usize> {
        let mut sel = self.default;
        loop {
            for (idx, item) in self.items.iter().enumerate() {
                term.write_line(&format!(
                    "{} {}",
                    if sel == idx {
                        ">"
                    } else {
                        " "
                    },
                    item,
                ))?;
            }
            match term.read_key()? {
                Key::ArrowDown | Key::Char('j') => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(self.items.len() as u64) as usize;
                    }
                }
                Key::ArrowUp | Key::Char('k') => {
                    if sel == !0 {
                        sel = self.items.len() - 1;
                    } else {
                        sel = ((sel as i64 - 1 + self.items.len() as i64) %
                               (self.items.len() as i64)) as usize;
                    }
                }
                Key::Enter | Key::Char(' ' ) if sel != !0 => {
                    if self.clear {
                        term.clear_last_lines(self.items.len())?;
                    }
                    return Ok(sel);
                }
                _ => {}
            }
            term.clear_last_lines(self.items.len())?;
        }
    }
}
