use std::{io, iter::repeat, ops::Rem};

use crate::{
    theme::{SimpleTheme, TermThemeRenderer, Theme},
    Paging,
};

use console::{Key, Term};

/// Renders a multi select prompt.
///
/// ## Example usage
/// ```rust,no_run
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// use dialoguer::MultiSelect;
///
/// let items = vec!["Option 1", "Option 2"];
/// let chosen : Vec<usize> = MultiSelect::new()
///     .items(&items)
///     .interact()?;
/// # Ok(())
/// # }
/// ```
pub struct MultiSelect<'a> {
    defaults: Vec<bool>,
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    theme: &'a dyn Theme,
}

impl<'a> Default for MultiSelect<'a> {
    fn default() -> MultiSelect<'a> {
        MultiSelect::new()
    }
}

impl<'a> MultiSelect<'a> {
    /// Creates a multi select prompt.
    pub fn new() -> MultiSelect<'static> {
        MultiSelect::with_theme(&SimpleTheme)
    }

    /// Creates a multi select prompt with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> MultiSelect<'a> {
        MultiSelect {
            items: vec![],
            defaults: vec![],
            clear: true,
            prompt: None,
            theme,
        }
    }

    /// Sets the clear behavior of the menu.
    ///
    /// The default is to clear the menu.
    pub fn clear(&mut self, val: bool) -> &mut MultiSelect<'a> {
        self.clear = val;
        self
    }

    /// Sets a defaults for the menu.
    pub fn defaults(&mut self, val: &[bool]) -> &mut MultiSelect<'a> {
        self.defaults = val
            .to_vec()
            .iter()
            .cloned()
            .chain(repeat(false))
            .take(self.items.len())
            .collect();
        self
    }

    /// Add a single item to the selector.
    #[inline]
    pub fn item<T: ToString>(&mut self, item: T) -> &mut MultiSelect<'a> {
        self.item_checked(item, false)
    }

    /// Add a single item to the selector with a default checked state.
    pub fn item_checked<T: ToString>(&mut self, item: T, checked: bool) -> &mut MultiSelect<'a> {
        self.items.push(item.to_string());
        self.defaults.push(checked);
        self
    }

    /// Adds multiple items to the selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut MultiSelect<'a> {
        for item in items {
            self.items.push(item.to_string());
            self.defaults.push(false);
        }
        self
    }

    /// Adds multiple items to the selector with checked state
    pub fn items_checked<T: ToString>(&mut self, items: &[(T, bool)]) -> &mut MultiSelect<'a> {
        for &(ref item, checked) in items {
            self.items.push(item.to_string());
            self.defaults.push(checked);
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after
    /// the selection.
    pub fn with_prompt<S: Into<String>>(&mut self, prompt: S) -> &mut MultiSelect<'a> {
        self.prompt = Some(prompt.into());
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items with the 'Space' bar and on 'Enter' the indices of selected items will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `Vec<index>` if user hit 'Enter'.
    /// This unlike [interact_opt](#method.interact_opt) does not allow to quit with 'Esc' or 'q'.
    #[inline]
    pub fn interact(&self) -> io::Result<Vec<usize>> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items with the 'Space' bar and on 'Enter' the indices of selected items will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `Some(Vec<index>)` if user hit 'Enter' or `None` if user cancelled with 'Esc' or 'q'.
    #[inline]
    pub fn interact_opt(&self) -> io::Result<Option<Vec<usize>>> {
        self.interact_on_opt(&Term::stderr())
    }

    /// Like [interact](#method.interact) but allows a specific terminal to be set.
    ///
    /// ## Examples
    ///```rust,no_run
    /// use dialoguer::Select;
    /// use console::Term;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let selections = MultiSelect::new()
    ///         .item("Option A")
    ///         .item("Option B")
    ///         .interact_on(&Term::stderr())?;
    ///
    ///     println!("User selected options at indices {:?}", selections);
    ///
    ///     Ok(())
    /// }
    ///```
    #[inline]
    pub fn interact_on(&self, term: &Term) -> io::Result<Vec<usize>> {
        self._interact_on(term, false)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case"))
    }

    /// Like [interact_opt](#method.interact_opt) but allows a specific terminal to be set.
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use dialoguer::Select;
    /// use console::Term;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let selections = MultiSelect::new()
    ///         .item("Option A")
    ///         .item("Option B")
    ///         .interact_on_opt(&Term::stdout())?;
    ///
    ///     match selections {
    ///         Some(positions) => println!("User selected options at indices {:?}", positions),
    ///         None => println!("User exited using Esc or q")
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn interact_on_opt(&self, term: &Term) -> io::Result<Option<Vec<usize>>> {
        self._interact_on(term, true)
    }

    fn _interact_on(&self, term: &Term, allow_quit: bool) -> io::Result<Option<Vec<usize>>> {
        if self.items.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Empty list of items given to `MultiSelect`",
            ));
        }

        let mut paging = Paging::new(term, self.items.len());
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = 0;

        let mut size_vec = Vec::new();

        for items in self
            .items
            .iter()
            .flat_map(|i| i.split('\n'))
            .collect::<Vec<_>>()
        {
            let size = &items.len();
            size_vec.push(*size);
        }

        let mut checked: Vec<bool> = self.defaults.clone();

        term.hide_cursor()?;

        loop {
            if let Some(ref prompt) = self.prompt {
                paging
                    .render_prompt(|paging_info| render.multi_select_prompt(prompt, paging_info))?;
            }

            for (idx, item) in self
                .items
                .iter()
                .enumerate()
                .skip(paging.current_page * paging.capacity)
                .take(paging.capacity)
            {
                render.multi_select_prompt_item(item, checked[idx], sel == idx)?;
            }

            term.flush()?;

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
                        sel = ((sel as i64 - 1 + self.items.len() as i64)
                            % (self.items.len() as i64)) as usize;
                    }
                }
                Key::ArrowLeft | Key::Char('h') => {
                    if paging.active {
                        sel = paging.previous_page();
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if paging.active {
                        sel = paging.next_page();
                    }
                }
                Key::Char(' ') => {
                    checked[sel] = !checked[sel];
                }
                Key::Escape | Key::Char('q') => {
                    if allow_quit {
                        if self.clear {
                            render.clear()?;
                        }

                        term.show_cursor()?;
                        term.flush()?;

                        return Ok(None);
                    }
                }
                Key::Enter => {
                    if self.clear {
                        render.clear()?;
                    }

                    if let Some(ref prompt) = self.prompt {
                        let selections: Vec<_> = checked
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, &checked)| {
                                if checked {
                                    Some(self.items[idx].as_str())
                                } else {
                                    None
                                }
                            })
                            .collect();

                        render.multi_select_prompt_selection(prompt, &selections[..])?;
                    }

                    term.show_cursor()?;
                    term.flush()?;

                    return Ok(Some(
                        checked
                            .into_iter()
                            .enumerate()
                            .filter_map(|(idx, checked)| if checked { Some(idx) } else { None })
                            .collect(),
                    ));
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
