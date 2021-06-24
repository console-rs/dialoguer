use crate::{
    theme::{SimpleTheme, TermThemeRenderer, Theme},
    Select,
};
use console::{Key, Term};
use fuzzy_matcher::FuzzyMatcher;
use std::{io, ops::Rem};

/// Renders a selection menu that user can fuzzy match to reduce set.
///
/// User can use fuzzy search to limit selectable items.
/// Interaction returns index of an item selected in the order they appear in `item` invocation or `items` slice.
///
/// ## Examples
///
/// ```rust,no_run
/// use dialoguer::{
///     FuzzySelect,
///     theme::ColorfulTheme
/// };
/// use console::Term;
///
/// fn main() -> std::io::Result<()> {
///     let items = vec!["Item 1", "item 2"];
///     let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
///         .items(&items)
///         .default(0)
///         .interact_on_opt(&Term::stderr())?;
///
///     match selection {
///         Some(index) => println!("User selected item : {}", items[index]),
///         None => println!("User did not select anything")
///     }
///
///     Ok(())
/// }
/// ```

// TODO: I don't think we need this. Ideally dialoguer should figure out the number of lines and intelligently do display them.
pub struct FuzzySelect<'a> {
    default: usize,
    items: Vec<String>,
    prompt: String,
    clear: bool,
    theme: &'a dyn Theme,
    paged: bool,
    offset: usize,
    lines_per_item: usize,
    fuzzy_search_is_case_sensitive: bool,
}

impl<'a> FuzzySelect<'a> {
    /// Creates the prompt with a specific text.
    pub fn new() -> Select<'static> {
        Select::with_theme(&SimpleTheme)
    }

    /// Same as `new` but with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> FuzzySelect<'a> {
        FuzzySelect {
            default: !0,
            items: vec![],
            prompt: "".into(),
            clear: true,
            theme,
            paged: false,
            offset: 1,
            lines_per_item: 1,
            fuzzy_search_is_case_sensitive: false,
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
        self.fuzzy_search_is_case_sensitive = val;
        self
    }

    /// Add a single item to the fuzzy selector.
    pub fn item<T: ToString>(&mut self, item: T) -> &mut FuzzySelect<'a> {
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
    pub fn with_prompt<S: Into<String>>(&mut self, prompt: S) -> &mut FuzzySelect<'a> {
        self.prompt = prompt.into();
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
        self.interact_on_opt(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<usize> {
        self._interact_on(term, false)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case"))
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on_opt(&self, term: &Term) -> io::Result<Option<usize>> {
        self._interact_on(term, true)
    }

    /// Like `interact` but allows a specific terminal to be set.
    fn _interact_on(&self, term: &Term, allow_quit: bool) -> io::Result<Option<usize>> {
        let mut page = 0;
        let mut position = 0;
        let mut capacity = self.items.len();
        let mut search_term = String::new();

        if self.paged {
            capacity = (term.size().0 as usize) / self.lines_per_item - self.offset;
        }

        let pages = (self.items.len() as f64 / capacity as f64).ceil() as usize;
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = self.default;

        let mut size_vec = Vec::new();
        for items in self.items.iter().as_slice() {
            let size = &items.len();
            size_vec.push(size.clone());
        }

        // Fuzzy matcher
        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

        loop {
            render.clear()?;
            render.fuzzy_select_prompt(self.prompt.as_str(), &search_term)?;
            term.hide_cursor()?;

            // Maps all items to a tuple of item and its match score.
            let mut filtered_list = self
                .items
                .iter()
                .map(|item| (item, matcher.fuzzy_match(item, &search_term)))
                .filter_map(|(item, score)| score.map(|s| (item, s)))
                .collect::<Vec<_>>();

            // Renders all matching items, from best match to worst.
            filtered_list.sort_unstable_by(|(_, s1), (_, s2)| s2.cmp(&s1));
            capacity = filtered_list.len();

            if self.paged {
                capacity = (term.size().0 as usize) / self.lines_per_item - self.offset;
            }

            for (idx, (item, _)) in filtered_list
                .iter()
                .enumerate()
                .skip(page * capacity)
                .take(capacity)
            {
                render.select_prompt_item(item, idx == sel)?;
                term.flush()?
            }

            match term.read_key()? {
                Key::Escape if allow_quit => {
                    if self.clear {
                        term.clear_last_lines(filtered_list.len())?;
                        term.flush()?
                    }
                    return Ok(None);
                }
                Key::ArrowUp if filtered_list.len() > 0 => {
                    if sel == !0 {
                        sel = filtered_list.len() - 1;
                    } else {
                        sel = ((sel as i64 - 1 + filtered_list.len() as i64)
                            % (filtered_list.len() as i64)) as usize;
                    }
                }
                Key::ArrowDown if filtered_list.len() > 0 => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(filtered_list.len() as u64) as usize;
                    }
                }
                Key::ArrowLeft if position > 0 => {
                    term.move_cursor_left(1)?;
                    position -= 1;
                    term.flush()?;
                }
                Key::ArrowRight if position < search_term.len() => {
                    term.move_cursor_right(1)?;
                    position += 1;
                    term.flush()?;
                }
                Key::Enter if filtered_list.len() > 0 => {
                    if self.clear {
                        render.clear()?;
                    }

                    
                    render.input_prompt_selection(self.prompt.as_str(), &filtered_list[sel].0)?;

                    let sel_string = filtered_list[sel].0;
                    let sel_string_pos_in_items =
                        self.items.iter().position(|item| item.eq(sel_string));

                    return Ok(sel_string_pos_in_items);
                }
                Key::Backspace if position > 0 => {
                    position -= 1;
                    search_term.remove(position);
                    term.clear_chars(1)?;

                    let tail = search_term[position..].to_string();

                    if !tail.is_empty() {
                        term.write_str(&tail)?;
                        term.move_cursor_left(tail.len())?;
                    }

                    term.flush()?;
                }
                Key::Char(chr) if !chr.is_ascii_control() => {
                    if self.fuzzy_search_is_case_sensitive {
                        search_term.insert(position, chr);
                    } else {
                        search_term.insert(position, chr.to_lowercase().to_string().pop().unwrap());
                    }

                    position += 1;

                    let tail = search_term[position..].to_string();

                    if !tail.is_empty() {
                        term.write_str(&tail)?;
                        term.move_cursor_left(tail.len() - 1)?;
                    }

                    term.flush()?;

                    sel = 0;
                }

                _ => {}
            }
            if filtered_list.len() > 0 && (sel < page * capacity || sel >= (page + 1) * capacity) {
                page = sel / capacity;
            }
        }
    }
}
