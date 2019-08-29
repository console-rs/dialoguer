use std::io;
use std::iter::repeat;
use std::ops::Rem;

use theme::{get_default_theme, SelectionStyle, TermThemeRenderer, Theme};

use console::{Key, Term};

/// Renders a selection menu.
pub struct Select<'a> {
    default: usize,
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    theme: &'a Theme,
    paged: bool,
}

/// Renders a multi select checkbox menu.
pub struct Checkboxes<'a> {
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    theme: &'a Theme,
    paged: bool,
}

impl<'a> Select<'a> {
    /// Creates the prompt with a specific text.
    pub fn new() -> Select<'static> {
        Select::with_theme(get_default_theme())
    }

    /// Same as `new` but with a specific theme.
    pub fn with_theme(theme: &'a Theme) -> Select<'a> {
        Select {
            default: !0,
            items: vec![],
            prompt: None,
            clear: true,
            theme: theme,
            paged: false,
        }
    }
    /// Enables or disables paging
    pub fn paged(&mut self, val: bool) -> &mut Select<'a> {
        self.paged = val;
        self
    }
    /// Sets the clear behavior of the menu.
    ///
    /// The default is to clear the menu.
    pub fn clear(&mut self, val: bool) -> &mut Select<'a> {
        self.clear = val;
        self
    }

    /// Sets a default for the menu
    pub fn default(&mut self, val: usize) -> &mut Select<'a> {
        self.default = val;
        self
    }

    /// Add a single item to the selector.
    pub fn item(&mut self, item: &str) -> &mut Select<'a> {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Select<'a> {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after
    /// the selection.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut Select<'a> {
        self.prompt = Some(prompt.to_string());
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The index of the selected item.
    /// The dialog is rendered on stderr.
    pub fn interact(&self) -> io::Result<usize> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// The index of the selected item. None if the user
    /// cancelled with Esc or 'q'.
    /// The dialog is rendered on stderr.
    pub fn interact_opt(&self) -> io::Result<Option<usize>> {
        self._interact_on(&Term::stderr(), true)
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<usize> {
        self._interact_on(term, false)?.ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Quit not allowed in this case",
        ))
    }

    /// Like `interact_opt` but allows a specific terminal to be set.
    pub fn interact_on_opt(&self, term: &Term) -> io::Result<Option<usize>> {
        self._interact_on(term, true)
    }

    /// Like `interact` but allows a specific terminal to be set.
    fn _interact_on(&self, term: &Term, allow_quit: bool) -> io::Result<Option<usize>> {
        let mut page = 0;
        let mut capacity = self.items.len();
        if self.paged {
            capacity = term.size().0 as usize - 1;
        }
        let pages = (self.items.len() / capacity) + 1;
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = self.default;
        if let Some(ref prompt) = self.prompt {
            render.prompt(prompt)?;
        }
        let mut size_vec = Vec::new();
        for items in self.items.iter().as_slice() {
            let size = &items.len();
            size_vec.push(size.clone());
        }
        loop {
            for (idx, item) in self
                .items
                .iter()
                .enumerate()
                .skip(page * capacity)
                .take(capacity)
            {
                render.selection(
                    item,
                    if sel == idx {
                        SelectionStyle::MenuSelected
                    } else {
                        SelectionStyle::MenuUnselected
                    },
                )?;
            }
            match term.read_key()? {
                Key::ArrowDown | Key::Char('j') => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(self.items.len() as u64) as usize;
                    }
                }
                Key::Escape | Key::Char('q') => {
                    if allow_quit {
                        if self.clear {
                            term.clear_last_lines(self.items.len())?;
                        }
                        return Ok(None);
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
                    if self.paged {
                        if page == 0 {
                            page = pages - 1;
                        } else {
                            page = page - 1;
                        }
                        sel = page * capacity;
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if self.paged {
                        if page == pages - 1 {
                            page = 0;
                        } else {
                            page = page + 1;
                        }
                        sel = page * capacity;
                    }
                }

                Key::Enter | Key::Char(' ') if sel != !0 => {
                    if self.clear {
                        render.clear()?;
                    }
                    if let Some(ref prompt) = self.prompt {
                        render.single_prompt_selection(prompt, &self.items[sel])?;
                    }
                    return Ok(Some(sel));
                }
                _ => {}
            }
            if sel < page * capacity || sel >= (page + 1) * capacity {
                page = sel / capacity;
            }
            render.clear_preserve_prompt(&size_vec)?;
        }
    }
}

impl<'a> Checkboxes<'a> {
    /// Creates a new checkbox object.
    pub fn new() -> Checkboxes<'static> {
        Checkboxes::with_theme(get_default_theme())
    }

    /// Sets a theme other than the default one.
    pub fn with_theme(theme: &'a Theme) -> Checkboxes<'a> {
        Checkboxes {
            items: vec![],
            clear: true,
            prompt: None,
            theme: theme,
            paged: false,
        }
    }
    /// Enables or disables paging
    pub fn paged(&mut self, val: bool) -> &mut Checkboxes<'a> {
        self.paged = val;
        self
    }
    /// Sets the clear behavior of the checkbox menu.
    ///
    /// The default is to clear the checkbox menu.
    pub fn clear(&mut self, val: bool) -> &mut Checkboxes<'a> {
        self.clear = val;
        self
    }

    /// Add a single item to the selector.
    pub fn item(&mut self, item: &str) -> &mut Checkboxes<'a> {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Checkboxes<'a> {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after
    /// the selection.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut Checkboxes<'a> {
        self.prompt = Some(prompt.to_string());
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
        let mut page = 0;
        let mut capacity = self.items.len();
        if self.paged {
            capacity = term.size().0 as usize - 1;
        }
        let pages = (self.items.len() / capacity) + 1;
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = 0;
        if let Some(ref prompt) = self.prompt {
            render.prompt(prompt)?;
        }
        let mut size_vec = Vec::new();
        for items in self.items.iter().as_slice() {
            let size = &items.len();
            size_vec.push(size.clone());
        }
        let mut checked: Vec<_> = repeat(false).take(self.items.len()).collect();
        loop {
            for (idx, item) in self
                .items
                .iter()
                .enumerate()
                .skip(page * capacity)
                .take(capacity)
            {
                render.selection(
                    item,
                    match (checked[idx], sel == idx) {
                        (true, true) => SelectionStyle::CheckboxCheckedSelected,
                        (true, false) => SelectionStyle::CheckboxCheckedUnselected,
                        (false, true) => SelectionStyle::CheckboxUncheckedSelected,
                        (false, false) => SelectionStyle::CheckboxUncheckedUnselected,
                    },
                )?;
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
                        sel = ((sel as i64 - 1 + self.items.len() as i64)
                            % (self.items.len() as i64)) as usize;
                    }
                }
                Key::ArrowLeft | Key::Char('h') => {
                    if self.paged {
                        if page == 0 {
                            page = pages - 1;
                        } else {
                            page = page - 1;
                        }
                        sel = page * capacity;
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if self.paged {
                        if page == pages - 1 {
                            page = 0;
                        } else {
                            page = page + 1;
                        }
                        sel = page * capacity;
                    }
                }
                Key::Char(' ') => {
                    checked[sel] = !checked[sel];
                }
                Key::Escape => {
                    if self.clear {
                        render.clear()?;
                    }
                    if let Some(ref prompt) = self.prompt {
                        render.multi_prompt_selection(prompt, &[][..])?;
                    }
                    return Ok(vec![]);
                }
                Key::Enter => {
                    if self.clear {
                        render.clear()?;
                    }
                    if let Some(ref prompt) = self.prompt {
                        let mut selections: Vec<_> = checked
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
                        render.multi_prompt_selection(prompt, &selections[..])?;
                    }
                    return Ok(checked
                        .into_iter()
                        .enumerate()
                        .filter_map(|(idx, checked)| if checked { Some(idx) } else { None })
                        .collect());
                }
                _ => {}
            }
            if sel < page * capacity || sel >= (page + 1) * capacity {
                page = sel / capacity;
            }
            render.clear_preserve_prompt(&size_vec)?;
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

        assert_eq!(
            Select::new().default(0).items(&selections[..]).items,
            selections
        );
    }

    #[test]
    fn test_string() {
        let selections = vec!["a".to_string(), "b".to_string()];

        assert_eq!(
            Select::new().default(0).items(&selections[..]).items,
            selections
        );
    }

    #[test]
    fn test_ref_str() {
        let a = "a";
        let b = "b";

        let selections = &[a, b];

        assert_eq!(
            Select::new().default(0).items(&selections[..]).items,
            selections
        );
    }
}
