use crate::theme::{SimpleTheme, TermThemeRenderer, Theme};
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

pub struct FuzzySelect<'a> {
    default: usize,
    items: Vec<String>,
    prompt: String,
    clear: bool,
    theme: &'a dyn Theme,
}

impl<'a> FuzzySelect<'a> {
    /// Creates the prompt with a specific text.
    pub fn new() -> FuzzySelect<'static> {
        FuzzySelect::with_theme(&SimpleTheme)
    }

    /// Same as `new` but with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> FuzzySelect<'a> {
        FuzzySelect {
            default: !0,
            items: vec![],
            prompt: "".into(),
            clear: true,
            theme,
        }
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
    /// The user can select the items using 'Enter' and the index of selected item will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `index` of selected item if user hit 'Enter'.
    /// This unlike [interact_opt](#method.interact_opt) does not allow to quit with 'Esc' or 'q'.
    #[inline]
    pub fn interact(&self) -> io::Result<usize> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items using 'Enter' and the index of selected item will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `Some(index)` if user hit 'Enter' or `None` if user cancelled with 'Esc' or 'q'.
    #[inline]
    pub fn interact_opt(&self) -> io::Result<Option<usize>> {
        self.interact_on_opt(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on(&self, term: &Term) -> io::Result<usize> {
        self._interact_on(term, false)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case"))
    }

    /// Like `interact` but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on_opt(&self, term: &Term) -> io::Result<Option<usize>> {
        self._interact_on(term, true)
    }

    /// Like `interact` but allows a specific terminal to be set.
    fn _interact_on(&self, term: &Term, allow_quit: bool) -> io::Result<Option<usize>> {
        let mut position = 0;
        let mut search_term = String::new();

        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = self.default;

        let mut size_vec = Vec::new();
        for items in self.items.iter().as_slice() {
            let size = &items.len();
            size_vec.push(size.clone());
        }

        // Fuzzy matcher
        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

        term.hide_cursor()?;

        loop {
            render.clear()?;
            render.fuzzy_select_prompt(self.prompt.as_str(), &search_term, position)?;

            // Maps all items to a tuple of item and its match score.
            let mut filtered_list = self
                .items
                .iter()
                .map(|item| (item, matcher.fuzzy_match(item, &search_term)))
                .filter_map(|(item, score)| score.map(|s| (item, s)))
                .collect::<Vec<_>>();

            // Renders all matching items, from best match to worst.
            filtered_list.sort_unstable_by(|(_, s1), (_, s2)| s2.cmp(&s1));

            for (idx, (item, _)) in filtered_list.iter().enumerate() {
                render.select_prompt_item(item, idx == sel)?;
                term.flush()?;
            }

            match term.read_key()? {
                Key::Escape if allow_quit => {
                    if self.clear {
                        term.clear_last_lines(filtered_list.len())?;
                        term.flush()?;
                    }
                    term.show_cursor()?;
                    return Ok(None);
                }
                Key::ArrowUp if filtered_list.len() > 0 => {
                    if sel == !0 {
                        sel = filtered_list.len() - 1;
                    } else {
                        sel = ((sel as i64 - 1 + filtered_list.len() as i64)
                            % (filtered_list.len() as i64)) as usize;
                    }
                    term.flush()?;
                }
                Key::ArrowDown if filtered_list.len() > 0 => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(filtered_list.len() as u64) as usize;
                    }
                    term.flush()?;
                }
                Key::ArrowLeft if position > 0 => {
                    position -= 1;
                    term.flush()?;
                }
                Key::ArrowRight if position < search_term.len() => {
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

                    term.show_cursor()?;
                    return Ok(sel_string_pos_in_items);
                }
                Key::Backspace if position > 0 => {
                    position -= 1;
                    search_term.remove(position);
                    term.flush()?;
                }
                Key::Char(chr) if !chr.is_ascii_control() => {
                    search_term.insert(position, chr);
                    position += 1;
                    term.flush()?;
                    sel = 0;
                }

                _ => {}
            }

            render.clear_preserve_prompt(&size_vec)?;
        }
    }
}
