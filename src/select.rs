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
    theme: &'a dyn Theme,
    paged: bool,
    offset: usize,
    lines_per_item: usize,
}

/// Renders a multi select checkbox menu.
pub struct Checkboxes<'a> {
    defaults: Vec<bool>,
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    theme: &'a dyn Theme,
    paged: bool,
    offset: usize,
    lines_per_item: usize,
}

/// Renders a selection menu that user can fuzzy match to reduce set.
pub struct FuzzySelect<'a> {
    default: usize,
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    theme: &'a dyn Theme,
    paged: bool,
    offset: usize,
    lines_per_item: usize,
    ignore_casing: bool,
    show_match: bool,
}

/// Renders a list to order.
pub struct OrderList<'a> {
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    theme: &'a dyn Theme,
    paged: bool,
}

impl<'a> Default for Select<'a> {
    fn default() -> Select<'a> {
        Select::new()
    }
}

impl<'a> Select<'a> {
    /// Creates the prompt with a specific text.
    pub fn new() -> Select<'static> {
        Select::with_theme(get_default_theme())
    }

    /// Same as `new` but with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> Select<'a> {
        Select {
            default: !0,
            items: vec![],
            prompt: None,
            clear: true,
            theme,
            paged: false,
            offset: 1,
            lines_per_item: 1,
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

    /// Sets number of lines paged offset includes
    pub fn offset(&mut self, val: usize) -> &mut Select<'a> {
        self.offset = val;
        self
    }

    /// Enables or disables paging
    pub fn lines_per_item(&mut self, val: usize) -> &mut Select<'a> {
        self.lines_per_item = val;
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
        self._interact_on(term, false)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case"))
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
            capacity = (term.size().0 as usize) / self.lines_per_item - self.offset;
        }
        let pages = (self.items.len() / capacity) + 1;
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = self.default;
        if let Some(ref prompt) = self.prompt {
            render.prompt(prompt)?;
        }
        let mut size_vec = Vec::new();
        for items in self
            .items
            .iter()
            .flat_map(|i| i.split('\n'))
            .collect::<Vec<_>>()
        {
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
                            page -= 1;
                        }
                        sel = page * capacity;
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if self.paged {
                        if page == pages - 1 {
                            page = 0;
                        } else {
                            page -= 1;
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
            if sel != !0 && (sel < page * capacity || sel >= (page + 1) * capacity) {
                page = sel / capacity;
            }
            render.clear_preserve_prompt(&size_vec)?;
        }
    }
}

impl<'a> Default for Checkboxes<'a> {
    fn default() -> Checkboxes<'a> {
        Checkboxes::new()
    }
}

impl<'a> Checkboxes<'a> {
    /// Creates a new checkbox object.
    pub fn new() -> Checkboxes<'static> {
        Checkboxes::with_theme(get_default_theme())
    }

    /// Sets a theme other than the default one.
    pub fn with_theme(theme: &'a dyn Theme) -> Checkboxes<'a> {
        Checkboxes {
            items: vec![],
            defaults: vec![],
            clear: true,
            prompt: None,
            theme,
            paged: false,
            offset: 1,
            lines_per_item: 1,
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

    /// Sets number of lines paged offset includes
    pub fn offset(&mut self, val: usize) -> &mut Checkboxes<'a> {
        self.offset = val;
        self
    }

    /// Enables or disables paging
    pub fn lines_per_item(&mut self, val: usize) -> &mut Checkboxes<'a> {
        self.lines_per_item = val;
        self
    }

    /// Sets a defaults for the menu
    pub fn defaults(&mut self, val: &[bool]) -> &mut Checkboxes<'a> {
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
    pub fn item(&mut self, item: &str) -> &mut Checkboxes<'a> {
        self.item_checked(item, false)
    }

    /// Add a single item to the selector with a default checked state.
    pub fn item_checked(&mut self, item: &str, checked: bool) -> &mut Checkboxes<'a> {
        self.items.push(item.to_string());
        self.defaults.push(checked);
        self
    }

    /// Adds multiple items to the selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Checkboxes<'a> {
        for item in items {
            self.items.push(item.to_string());
            self.defaults.push(false);
        }
        self
    }

    /// Adds multiple items to the selector with checked state
    pub fn items_checked<T: ToString>(&mut self, items: &[(T, bool)]) -> &mut Checkboxes<'a> {
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
            capacity = (term.size().0 as usize) / self.lines_per_item - self.offset;
        }
        let pages = (self.items.len() / capacity) + 1;
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = 0;
        if let Some(ref prompt) = self.prompt {
            render.prompt(prompt)?;
        }
        let mut size_vec = Vec::new();
        for items in self
            .items
            .iter()
            .flat_map(|i| i.split('\n'))
            .collect::<Vec<_>>()
        {
            let size = &items.len();
            size_vec.push(size.clone());
        }
        let mut checked: Vec<bool> = self.defaults.clone();
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
                            page -= 1;
                        }
                        sel = page * capacity;
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if self.paged {
                        if page == pages - 1 {
                            page = 0;
                        } else {
                            page += 1;
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
                    return Ok(self
                        .defaults
                        .clone()
                        .into_iter()
                        .enumerate()
                        .filter_map(|(idx, checked)| if checked { Some(idx) } else { None })
                        .collect());
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

impl<'a> FuzzySelect<'a> {
    /// Creates the prompt with a specific text.
    pub fn new() -> Select<'static> {
        Select::with_theme(get_default_theme())
    }

    /// Same as `new` but with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> FuzzySelect<'a> {
        FuzzySelect {
            default: !0,
            items: vec![],
            prompt: None,
            clear: true,
            theme: theme,
            paged: false,
            offset: 1,
            lines_per_item: 1,
            ignore_casing: true,
            show_match: false,
        }
    }
    /// Enables or disables paging
    pub fn paged(&mut self, val: bool) -> &mut FuzzySelect<'a> {
        self.paged = val;
        self
    }
    /// Sets the clear behavior of the menu.
    ///
    /// The default is to clear the menu.
    pub fn clear(&mut self, val: bool) -> &mut FuzzySelect<'a> {
        self.clear = val;
        self
    }

    /// Sets a default for the menu
    pub fn default(&mut self, val: usize) -> &mut FuzzySelect<'a> {
        self.default = val;
        self
    }

    /// Sets number of lines paged offset includes
    pub fn offset(&mut self, val: usize) -> &mut FuzzySelect<'a> {
        self.offset = val;
        self
    }

    /// Enables or disables paging
    pub fn lines_per_item(&mut self, val: usize) -> &mut FuzzySelect<'a> {
        self.lines_per_item = val;
        self
    }

    /// Specify whether casing should be ignored in matches
    pub fn ignore_casing(&mut self, val: bool) -> &mut FuzzySelect<'a> {
        self.ignore_casing = val;
        self
    }

    /// Specify whether match string is shown as typed
    pub fn show_match(&mut self, val: bool) -> &mut FuzzySelect<'a> {
        self.show_match = val;
        self
    }

    /// Add a single item to the fuzzy selector.
    pub fn item(&mut self, item: &str) -> &mut FuzzySelect<'a> {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the fuzzy selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut FuzzySelect<'a> {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after
    /// the fuzzy selection.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut FuzzySelect<'a> {
        self.prompt = Some(prompt.to_string());
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The index of the selected item.
    /// The dialog is rendered on stderr.
    pub fn interact(&self) -> io::Result<String> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// The index of the selected item. None if the user
    /// cancelled with Esc or 'q'.
    /// The dialog is rendered on stderr.
    pub fn interact_opt(&self) -> io::Result<Option<String>> {
        self._interact_on(&Term::stderr(), true)
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<String> {
        self._interact_on(term, false)?.ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Quit not allowed in this case",
        ))
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on_opt(&self, term: &Term) -> io::Result<Option<String>> {
        self._interact_on(term, true)
    }

    /// Like `interact` but allows a specific terminal to be set.
    fn _interact_on(&self, term: &Term, allow_quit: bool) -> io::Result<Option<String>> {
        let mut page = 0;
        let mut capacity = self.items.len();
        let mut search_term = String::new();
        if self.paged {
            capacity = (term.size().0 as usize) / self.lines_per_item - self.offset;
        }
        if self.show_match {
            capacity -= 1;
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
            let filtered_list: Vec<&String> = self
                .items
                .iter()
                .filter(|item| {
                    if self.ignore_casing {
                        item.to_lowercase().find(&search_term).is_some()
                    } else {
                        item.find(&search_term).is_some()
                    }
                })
                .collect();

            capacity = filtered_list.len();
            if self.paged {
                capacity = (term.size().0 as usize) / self.lines_per_item - self.offset;
            }
            if self.show_match {
                term.write_line(&search_term)?;
            }

            for (idx, item) in filtered_list
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
                Key::ArrowDown => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(filtered_list.len() as u64) as usize;
                    }
                }
                Key::Escape => {
                    if allow_quit {
                        if self.clear {
                            term.clear_last_lines(filtered_list.len())?;
                        }
                        return Ok(None);
                    }
                }
                Key::ArrowUp if filtered_list.len() > 0 => {
                    if sel == !0 {
                        sel = filtered_list.len() - 1;
                    } else {
                        sel = ((sel as i64 - 1 + filtered_list.len() as i64)
                            % (filtered_list.len() as i64)) as usize;
                    }
                }
                Key::ArrowLeft => {
                    if self.paged {
                        if page == 0 {
                            page = pages - 1;
                        } else {
                            page = page - 1;
                        }
                        sel = page * capacity;
                    }
                }
                Key::ArrowRight => {
                    if self.paged {
                        if page == pages - 1 {
                            page = 0;
                        } else {
                            page = page + 1;
                        }
                        sel = page * capacity;
                    }
                }

                Key::Enter if filtered_list.len() > 0 => {
                    if self.clear {
                        render.clear()?;
                    }
                    if let Some(ref prompt) = self.prompt {
                        render.single_prompt_selection(prompt, &filtered_list[sel])?;
                    }
                    return Ok(Some(filtered_list[sel].to_owned()));
                }
                Key::Backspace => {
                    search_term.pop();
                }
                Key::Char(key) => {
                    if self.ignore_casing {
                        search_term.push(key.to_lowercase().to_string().pop().unwrap());
                    } else {
                        search_term.push(key);
                    }
                    sel = 0;
                }
                _ => {}
            }
            if filtered_list.len() > 0 && (sel < page * capacity || sel >= (page + 1) * capacity) {
                page = sel / capacity;
            }
            render.clear_preserve_prompt(&size_vec)?;
            if self.show_match {
                term.clear_last_lines(1)?;
            }
        }
    }
}

impl<'a> Default for OrderList<'a> {
    fn default() -> OrderList<'a> {
        OrderList::new()
    }
}

impl<'a> OrderList<'a> {
    /// Creates a new orderlist object.
    pub fn new() -> OrderList<'static> {
        OrderList::with_theme(get_default_theme())
    }

    /// Sets a theme other than the default one.
    pub fn with_theme(theme: &'a dyn Theme) -> OrderList<'a> {
        OrderList {
            items: vec![],
            clear: true,
            prompt: None,
            theme,
            paged: false,
        }
    }
    /// Enables or disables paging
    pub fn paged(&mut self, val: bool) -> &mut OrderList<'a> {
        self.paged = val;
        self
    }
    /// Sets the clear behavior of the checkbox menu.
    ///
    /// The default is to clear the checkbox menu.
    pub fn clear(&mut self, val: bool) -> &mut OrderList<'a> {
        self.clear = val;
        self
    }

    /// Add a single item to the selector.
    pub fn item(&mut self, item: &str) -> &mut OrderList<'a> {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut OrderList<'a> {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after

    /// the selection.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut OrderList<'a> {
        self.prompt = Some(prompt.to_string());
        self
    }

    /// The user can order the items with the space bar and the arrows.
    /// On enter the ordered list will be returned.
    pub fn interact(&self) -> io::Result<Vec<usize>> {
        self.interact_on(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<Vec<usize>> {
        let mut page = 0;
        let capacity = if self.paged {
            term.size().0 as usize - 1
        } else {
            self.items.len()
        };
        let pages = (self.items.len() as f64 / capacity as f64).ceil() as usize;
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
        let mut order: Vec<_> = (0..self.items.len()).collect();
        let mut checked: bool = false;
        loop {
            for (idx, item) in order
                .iter()
                .enumerate()
                .skip(page * capacity)
                .take(capacity)
            {
                render.selection(
                    &self.items[*item],
                    match (sel == idx, checked) {
                        (true, true) => SelectionStyle::CheckboxCheckedSelected,
                        (true, false) => SelectionStyle::CheckboxUncheckedSelected,
                        (false, _) => SelectionStyle::CheckboxUncheckedUnselected,
                    },
                )?;
            }
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
                    if self.paged {
                        let old_sel = sel;
                        let old_page = page;
                        if page == 0 {
                            page = pages - 1;
                        } else {
                            page -= 1;
                        }
                        sel = page * capacity;
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
                    if self.paged {
                        let old_sel = sel;
                        let old_page = page;
                        if page == pages - 1 {
                            page = 0;
                        } else {
                            page += 1;
                        }
                        sel = page * capacity;
                        if checked {
                            let indexes: Vec<_> = if old_page == pages - 1 {
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
                        render.multi_prompt_selection(prompt, &list[..])?;
                    }
                    return Ok(order);
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
