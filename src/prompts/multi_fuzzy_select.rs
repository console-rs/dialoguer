use crate::theme::{SimpleTheme, TermThemeRenderer, Theme};
use console::{Key, Term};
use fuzzy_matcher::FuzzyMatcher;
use std::iter::repeat;
use std::{io, ops::Rem};

/// Renders a multi selection menu that user can fuzzy match to reduce set.
/// Selection/Deselection is done by pressing `Spacebar`.
/// (Note that this restricts the user to only search non-spacebar characters)
///
/// User can use fuzzy search to limit selectable items.
/// Interaction returns `Vec` of indices of the selected items in the order they appear in `item` invocation or `items` slice.
///
/// ## Examples
///
/// ```rust,no_run
/// use dialoguer::{
///     MultiFuzzySelect,
///     theme::ColorfulTheme
/// };
/// use console::Term;
///
/// fn main() -> std::io::Result<()> {
///     let items = vec!["Item 1", "item 2"];
///     let selection = MultiFuzzySelect::with_theme(&ColorfulTheme::default())
///         .items(&items)
///         .default(0)
///         .interact_on_opt(&Term::stderr())?;
///
///     match selection {
///         Some(indices) => println!("User selected items : {:?}", indices),
///         None => println!("User did not select anything")
///     }
///
///     Ok(())
/// }
/// ```
pub struct MultiFuzzySelect<'a> {
    defaults: Vec<bool>,
    items: Vec<String>,
    prompt: String,
    report: bool,
    clear: bool,
    highlight_matches: bool,
    max_length: Option<usize>,
    theme: &'a dyn Theme,
}

impl Default for MultiFuzzySelect<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFuzzySelect<'static> {
    /// Creates the prompt with a specific text.
    pub fn new() -> Self {
        Self::with_theme(&SimpleTheme)
    }
}

impl MultiFuzzySelect<'_> {
    /// Sets the clear behavior of the menu.
    ///
    /// The default is to clear the menu.
    pub fn clear(&mut self, val: bool) -> &mut Self {
        self.clear = val;
        self
    }

    /// Sets a default selection for the menu
    pub fn defaults(&mut self, val: &[bool]) -> &mut Self {
        self.defaults = val
            .to_vec()
            .iter()
            .copied()
            .chain(repeat(false))
            .take(self.items.len())
            .collect();
        self
    }

    /// Add a single item to the fuzzy selector.
    pub fn item<T: ToString>(&mut self, item: T) -> &mut Self {
        self.items.push(item.to_string());
        self.defaults.push(false);
        self
    }

    /// Adds multiple items to the fuzzy selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Self {
        for item in items {
            self.items.push(item.to_string());
            self.defaults.push(false);
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after
    /// the fuzzy selection.
    pub fn with_prompt<S: Into<String>>(&mut self, prompt: S) -> &mut Self {
        self.prompt = prompt.into();
        self
    }

    /// Indicates whether to report the selected value after interaction.
    ///
    /// The default is to report the selection.
    pub fn report(&mut self, val: bool) -> &mut Self {
        self.report = val;
        self
    }

    /// Indicates whether to highlight matched indices
    ///
    /// The default is to highlight the indices
    pub fn highlight_matches(&mut self, val: bool) -> &mut Self {
        self.highlight_matches = val;
        self
    }

    /// Sets the maximum number of visible options.
    ///
    /// The default is the height of the terminal minus 2.
    pub fn max_length(&mut self, rows: usize) -> &mut Self {
        self.max_length = Some(rows);
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can toggle the selection of the hovered item using 'Spacebar'.
    /// After the desired items are all toggled, the user can submit the selection by pressing
    /// 'Enter' and the indices of the selected items will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `Vec<index>` of selected items if user hit 'Enter'.
    /// This unlike [interact_opt](#method.interact_opt) does not allow to quit with 'Esc' or 'q'.
    #[inline]
    pub fn interact(&self) -> io::Result<Vec<usize>> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can toggle the selection of the hovered item using 'Spacebar'.
    /// After the desired items are all toggled, the user can submit the selection by pressing
    /// 'Enter' and the indices of the selected items will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `Some(Vec<index>)` if user hit 'Enter' or `None` if user cancelled with 'Esc' or 'q'.
    #[inline]
    pub fn interact_opt(&self) -> io::Result<Vec<usize>> {
        self.interact_on_opt(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on(&self, term: &Term) -> io::Result<Vec<usize>> {
        self._interact_on(term, false)
    }

    /// Like `interact_opt` but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on_opt(&self, term: &Term) -> io::Result<Vec<usize>> {
        self._interact_on(term, true)
    }

    fn _interact_on(&self, term: &Term, allow_quit: bool) -> io::Result<Vec<usize>> {
        let mut current_fuzzy_term_length = 0;
        let mut fuzzy_term = String::new();

        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut cursor_position = 0;

        let size_vec = self.items.iter().map(|item| item.len()).collect::<Vec<_>>();

        // Fuzzy matcher
        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

        // Subtract -2 because we need space to render the prompt.
        let visible_term_rows = (term.size().0 as usize).max(3) - 2;
        let visible_term_rows = self
            .max_length
            .map(|max_len| max_len.min(visible_term_rows))
            .unwrap_or(visible_term_rows);
        // Variable used to determine if we need to scroll through the list.
        let mut starting_row = 0;

        term.hide_cursor()?;

        let mut checked: Vec<bool> = self.defaults.clone();

        loop {
            render.clear()?;
            render.multi_fuzzy_select_prompt(
                self.prompt.as_str(),
                &fuzzy_term,
                current_fuzzy_term_length,
            )?;

            // Maps all items to a tuple of item and its match score.
            let mut filtered_list = self
                .items
                .iter()
                .enumerate()
                .map(|(idx, item)| (idx, item, matcher.fuzzy_match(item, &fuzzy_term)))
                .filter_map(|(idx, item, score)| score.map(|score_value| (idx, item, score_value)))
                .collect::<Vec<_>>();

            // Renders all matching items, from best match to worst.
            filtered_list.sort_unstable_by(|(_, _, score_1), (_, _, score_2)| {
                score_1.cmp(score_2).reverse()
            });

            // the cursor position cannot exceed the last element
            cursor_position = cursor_position.min(filtered_list.len().saturating_sub(1));

            for (idx, (item_idx, item, _)) in filtered_list
                .iter()
                .enumerate()
                .skip(starting_row)
                .take(visible_term_rows)
            {
                render.multi_fuzzy_select_prompt_item(
                    item,
                    idx == cursor_position,
                    checked[*item_idx],
                    self.highlight_matches,
                    &matcher,
                    &fuzzy_term,
                )?;
                term.flush()?;
            }

            match term.read_key()? {
                Key::Escape if allow_quit => {
                    if self.clear {
                        render.clear()?;
                        term.flush()?;
                    }
                    term.show_cursor()?;
                    return Ok(vec![]);
                }
                Key::ArrowUp | Key::BackTab if !filtered_list.is_empty() => {
                    if cursor_position == 0 {
                        // wrap around display window top to bottom
                        starting_row =
                            filtered_list.len().max(visible_term_rows) - visible_term_rows;
                    } else if cursor_position == starting_row {
                        // move display window up
                        starting_row -= 1;
                    }

                    // move cursor up taking into account wrap around
                    cursor_position = (cursor_position + filtered_list.len().saturating_sub(1))
                        .rem(filtered_list.len());

                    term.flush()?;
                }
                Key::ArrowDown | Key::Tab if !filtered_list.is_empty() => {
                    if cursor_position == filtered_list.len() - 1 {
                        // wrap around display window bottom to top
                        starting_row = 0;
                    } else if cursor_position == visible_term_rows + starting_row {
                        // move display window down
                        starting_row += 1;
                    }

                    // move cursor down taking into account wrap around
                    cursor_position = (cursor_position + 1).rem(filtered_list.len());

                    term.flush()?;
                }
                Key::ArrowLeft if current_fuzzy_term_length > 0 => {
                    current_fuzzy_term_length -= 1;
                    term.flush()?;
                }
                Key::ArrowRight if current_fuzzy_term_length < fuzzy_term.len() => {
                    current_fuzzy_term_length += 1;
                    term.flush()?;
                }
                Key::Enter if !filtered_list.is_empty() => {
                    if self.clear {
                        render.clear()?;
                    }

                    if self.report {
                        render.input_prompt_selection(
                            self.prompt.as_str(),
                            filtered_list[cursor_position].1,
                        )?;
                    }

                    let selected_items = checked
                        .into_iter()
                        .enumerate()
                        .filter_map(|(idx, checked)| checked.then(|| idx))
                        .collect::<Vec<_>>();

                    term.show_cursor()?;
                    return Ok(selected_items);
                }
                Key::Backspace if current_fuzzy_term_length > 0 => {
                    current_fuzzy_term_length -= 1;
                    fuzzy_term.remove(current_fuzzy_term_length);
                    term.flush()?;
                }
                Key::Char(' ') => {
                    if let Some(sel_string_pos_in_items) = filtered_list
                        .get(cursor_position)
                        .map(|(_, cursor_item_name, _)| cursor_item_name)
                        .and_then(|&cursor_item_name| {
                            self.items.iter().position(|item| item == cursor_item_name)
                        })
                    {
                        checked[sel_string_pos_in_items] = !checked[sel_string_pos_in_items];
                    }

                    fuzzy_term.clear();
                    current_fuzzy_term_length = 0;
                    term.flush()?;
                }
                Key::Char(chr) if !chr.is_ascii_control() => {
                    fuzzy_term.insert(current_fuzzy_term_length, chr);
                    current_fuzzy_term_length += 1;
                    term.flush()?;
                }

                _ => {}
            }

            render.clear_preserve_prompt(&size_vec)?;
        }
    }
}

impl<'a> MultiFuzzySelect<'a> {
    /// Same as `new` but with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            defaults: vec![],
            items: vec![],
            prompt: "".into(),
            report: true,
            clear: true,
            highlight_matches: true,
            max_length: None,
            theme,
        }
    }
}
