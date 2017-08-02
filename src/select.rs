use std::io;
use std::ops::Rem;
use std::iter::repeat;

use console::{Key, Term};

/// Renders a selection menu.
pub struct Select {
    default: usize,
    items: Vec<String>,
    clear: bool,
}

/// Renders a multi select checkbox menu.
pub struct Checkboxes {
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
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Select {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stderr.
    pub fn interact(&self) -> io::Result<usize> {
        self.interact_on(&Term::stderr())
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

impl Checkboxes {
    /// Creates the prompt with a specific text.
    pub fn new() -> Checkboxes {
        Checkboxes {
            items: vec![],
            clear: true,
        }
    }

    /// Sets the clear behavior of the checkbox menu.
    ///
    /// The default is to clear the checkbox menu.
    pub fn clear(&mut self, val: bool) -> &mut Checkboxes {
        self.clear = val;
        self
    }

    /// Add a single item to the selector.
    pub fn item(&mut self, item: &str) -> &mut Checkboxes {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the selector.
    pub fn items(&mut self, items: &[&str]) -> &mut Checkboxes {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items with the space bar and on enter
    /// the selected items will be returned.
    pub fn interact(&self) -> io::Result<Vec<usize>> {
        self.interact_on(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<Vec<usize>> {
        let mut sel = 0;
        let mut selected: Vec<_> = repeat(false).take(self.items.len()).collect();
        loop {
            for (idx, item) in self.items.iter().enumerate() {
                term.write_line(&format!(
                    "{} [{}] {}",
                    if sel == idx {
                        ">"
                    } else {
                        " "
                    },
                    if selected[idx] {
                        "x"
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
                Key::Char(' ' ) => {
                    selected[sel] = !selected[sel];
                }
                Key::Escape => {
                    if self.clear {
                        term.clear_last_lines(self.items.len())?;
                    }
                    return Ok(vec![]);
                },
                Key::Enter => {
                    if self.clear {
                        term.clear_last_lines(self.items.len())?;
                    }
                    return Ok(selected.into_iter().enumerate().filter_map(|(idx, selected)| {
                        if selected {
                            Some(idx)
                        } else {
                            None
                        }
                    }).collect());
                }
                _ => {}
            }
            term.clear_last_lines(self.items.len())?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        let selections = &[
            "Ice Cream",
            "Vanilla Cupcake",
            "Chocolate Muffin",
            "A Pile of sweet, sweet mustard",
        ];

        assert_eq!(Select::new().default(0).items(&selections[..]).items, selections);
    }

    #[test]
    fn test_string() {
        let selections = vec![
            "a".to_string(),
            "b".to_string()
        ];

        assert_eq!(Select::new().default(0).items(&selections[..]).items, selections);
    }

    #[test]
    fn test_ref_str() {
        let a = "a";
        let b = "b";

        let selections = &[
            a,
            b
        ];

        assert_eq!(Select::new().default(0).items(&selections[..]).items, selections);
    }
}
