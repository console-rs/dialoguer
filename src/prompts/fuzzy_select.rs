use crate::theme::{SimpleTheme, Theme, TermThemeRenderer};
use crate::Select;
use std::io;
use console::{Term, Key};
use fuzzy_matcher::FuzzyMatcher;
use std::ops::Rem;

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
            prompt: None,
            clear: true,
            theme,
            paged: false,
            offset: 1,
            lines_per_item: 1,
            ignore_casing: true,
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

        let pages = (self.items.len() / capacity) + 1;
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
            render.fuzzy_select_prompt(self.prompt.as_deref(), &search_term)?;

            // Maps all items to a tuple of item and its match score.
            let mut filtered_list = self.items.iter()
                .map(|item| (item, matcher.fuzzy_match(item, &search_term)))
                .filter_map(|(item, score)| match score {
                    Some(score) => Some((item, score)),
                    _ => None,
                })
                .collect::<Vec<_>>();

            // Renders all matching items, from best match to worst.
            filtered_list.sort_unstable_by(|(_, s1), (_, s2)| s2.cmp(&s1));
            capacity = filtered_list.len();
            if self.paged {
                capacity = (term.size().0 as usize) / self.lines_per_item - self.offset;
            }

            for (idx, (item, _)) in filtered_list.iter().enumerate().skip(page * capacity).take(capacity) {
                render.select_prompt_item(item, idx == sel)?;
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
                        render.input_prompt_selection(prompt, &filtered_list[sel].0)?;
                    }
                    return Ok(Some(filtered_list[sel].0.to_owned()));
                },
                Key::Backspace => {
                    search_term.pop();
                },
                Key::Char(key) => {
                    if self.ignore_casing {
                        search_term.push(key.to_lowercase().to_string().pop().unwrap());
                    } else {
                        search_term.push(key);
                    }
                    sel = 0;
                },
                _ => {}
            }
            if filtered_list.len() > 0 && (sel < page * capacity || sel >= (page + 1) * capacity) {
                page = sel / capacity;
            }
        }
    }
}