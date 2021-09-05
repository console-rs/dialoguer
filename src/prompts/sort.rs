use std::{io, ops::Rem};

use crate::{
    theme::{SimpleTheme, TermThemeRenderer, Theme},
    Paging,
};

use console::{Key, Term};

/// Renders a sort prompt.
///
/// Returns list of indices in original items list sorted according to user input.
///
/// ## Example usage
/// ```rust,no_run
/// use dialoguer::Sort;
///
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// let items_to_order = vec!["Item 1", "Item 2", "Item 3"];
/// let ordered = Sort::new()
///     .with_prompt("Order the items")
///     .items(&items_to_order)
///     .interact()?;
/// # Ok(())
/// # }
/// ```
pub struct Sort<'a> {
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    theme: &'a dyn Theme,
}

impl<'a> Default for Sort<'a> {
    fn default() -> Sort<'a> {
        Sort::new()
    }
}

impl<'a> Sort<'a> {
    /// Creates a sort prompt.
    pub fn new() -> Sort<'static> {
        Sort::with_theme(&SimpleTheme)
    }

    /// Creates a sort prompt with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> Sort<'a> {
        Sort {
            items: vec![],
            clear: true,
            prompt: None,
            theme,
        }
    }

    /// Sets the clear behavior of the menu.
    ///
    /// The default is to clear the menu after user interaction.
    pub fn clear(&mut self, val: bool) -> &mut Sort<'a> {
        self.clear = val;
        self
    }

    /// Add a single item to the selector.
    pub fn item<T: ToString>(&mut self, item: T) -> &mut Sort<'a> {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Sort<'a> {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after
    /// the selection.
    pub fn with_prompt<S: Into<String>>(&mut self, prompt: S) -> &mut Sort<'a> {
        self.prompt = Some(prompt.into());
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can order the items with the space bar and the arrows.
    /// On enter the ordered list will be returned.
    pub fn interact(&self) -> io::Result<Vec<usize>> {
        self.interact_on(&Term::stderr())
    }

    /// Like [interact](#method.interact) but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<Vec<usize>> {
        if self.items.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Empty list of items given to `Sort`",
            ));
        }

        let mut paging = Paging::new(term, self.items.len());
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = 0;

        let mut size_vec = Vec::new();

        for items in self.items.iter().as_slice() {
            let size = &items.len();
            size_vec.push(*size);
        }

        let mut order: Vec<_> = (0..self.items.len()).collect();
        let mut checked: bool = false;

        term.hide_cursor()?;

        loop {
            if let Some(ref prompt) = self.prompt {
                paging.render_prompt(|paging_info| render.sort_prompt(prompt, paging_info))?;
            }

            for (idx, item) in order
                .iter()
                .enumerate()
                .skip(paging.current_page * paging.capacity)
                .take(paging.capacity)
            {
                render.sort_prompt_item(&self.items[*item], checked, sel == idx)?;
            }

            term.flush()?;

            match term.read_key()? {
                Key::ArrowDown | Key::Char('j') => {
                    let old_sel = sel;

                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(self.items.len() as u64) as usize;
                    }

                    if checked && old_sel != sel {
                        order.swap(old_sel, sel);
                    }
                }
                Key::ArrowUp | Key::Char('k') => {
                    let old_sel = sel;

                    if sel == !0 {
                        sel = self.items.len() - 1;
                    } else {
                        sel = ((sel as i64 - 1 + self.items.len() as i64)
                            % (self.items.len() as i64)) as usize;
                    }

                    if checked && old_sel != sel {
                        order.swap(old_sel, sel);
                    }
                }
                Key::ArrowLeft | Key::Char('h') => {
                    if paging.active {
                        let old_sel = sel;
                        let old_page = paging.current_page;

                        sel = paging.previous_page();

                        if checked {
                            let indexes: Vec<_> = if old_page == 0 {
                                let indexes1: Vec<_> = (0..=old_sel).rev().collect();
                                let indexes2: Vec<_> = (sel..self.items.len()).rev().collect();
                                [indexes1, indexes2].concat()
                            } else {
                                (sel..=old_sel).rev().collect()
                            };

                            for index in 0..(indexes.len() - 1) {
                                order.swap(indexes[index], indexes[index + 1]);
                            }
                        }
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if paging.active {
                        let old_sel = sel;
                        let old_page = paging.current_page;

                        sel = paging.next_page();

                        if checked {
                            let indexes: Vec<_> = if old_page == paging.pages - 1 {
                                let indexes1: Vec<_> = (old_sel..self.items.len()).collect();
                                let indexes2: Vec<_> = vec![0];
                                [indexes1, indexes2].concat()
                            } else {
                                (old_sel..=sel).collect()
                            };

                            for index in 0..(indexes.len() - 1) {
                                order.swap(indexes[index], indexes[index + 1]);
                            }
                        }
                    }
                }
                Key::Char(' ') => {
                    checked = !checked;
                }
                // TODO: Key::Escape
                Key::Enter => {
                    if self.clear {
                        render.clear()?;
                    }

                    if let Some(ref prompt) = self.prompt {
                        let list: Vec<_> = order
                            .iter()
                            .enumerate()
                            .map(|(_, item)| self.items[*item].as_str())
                            .collect();
                        render.sort_prompt_selection(prompt, &list[..])?;
                    }

                    term.show_cursor()?;
                    term.flush()?;

                    return Ok(order);
                }
                _ => {}
            }

            paging.update(sel)?;

            if paging.active {
                render.clear()?;
            } else {
                render.clear_preserve_prompt(&size_vec)?;
            }
        }
    }
}
