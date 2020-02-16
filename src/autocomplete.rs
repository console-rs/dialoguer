use theme::{Theme, get_default_theme, TermThemeRenderer, SelectionStyle};
use Validator;

use std::io;
use std::ops::Rem;

use console::{Key, Term};
use std::io::Write;


pub struct Autocomplete<'a> {
    prompt: Option<String>,
    theme: &'a dyn Theme,
    validator: Option<Box<dyn Fn(&str) -> Option<String>>>,
    items: Vec<String>,
    paged: bool,
    clear: bool,
}

impl<'a> Default for Autocomplete<'a> {
    fn default() -> Self {
        Autocomplete::new()
    }
}

impl<'a> Autocomplete<'a> {
    pub fn new() -> Autocomplete<'static> {
        Autocomplete::with_theme(get_default_theme())
    }

    pub fn with_theme(theme: &'a dyn Theme) -> Autocomplete<'a> {
        Autocomplete {
            items: vec![],
            prompt: None,
            theme,
            paged: false,
            validator: None,
            clear: true
        }
    }

    /// Enables or disables paging
    pub fn paged(&mut self, val: bool) -> &mut Autocomplete<'a> {
        self.paged = val;
        self
    }

    /// Add a single item to the selector.
    pub fn item<T: ToString>(&mut self, item: &T) -> &mut Autocomplete<'a> {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the selector.
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Autocomplete<'a> {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after
    /// the selection.
    pub fn with_prompt(&mut self, prompt: &str) -> &mut Autocomplete<'a> {
        self.prompt = Some(prompt.to_string());
        self
    }

    /// Sets the clear behavior of the menu.
    ///
    /// The default is to clear the menu.
    pub fn clear(&mut self, val: bool) -> &mut Autocomplete<'a> {
        self.clear = val;
        self
    }

    /// Registers a validator.
    pub fn validate_with<V: Validator + 'static>(&mut self, validator: V) -> &mut Autocomplete<'a> {
        let old_validator_func = self.validator.take();
        self.validator = Some(Box::new(move |value: &str| -> Option<String> {
            if let Some(old) = old_validator_func.as_ref() {
                if let Some(err) = old(value) {
                    return Some(err);
                }
            }
            match validator.validate(value) {
                Ok(()) => None,
                Err(err) => Some(err.to_string()),
            }
        }));
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
        let capacity = if self.paged {
            term.size().0 as usize - 1
        } else {
            self.items.len()
        };
        let pages = (self.items.len() / capacity) + 1;
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = !0;

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

        let mut input = String::new();
        if let Some(ref prompt) = self.prompt {
            render.input_prompt(prompt, None)?;
        }
        loop {
            render.clear_preserve_prompt(&[input.len()]);
            render.add_line();
            term.write_line(&input);

            let filtered_items = if input.is_empty() {
                self.items.clone()
            } else {
                self.items
                    .iter()
                    .cloned()
                    .filter(|item| item.to_ascii_lowercase().contains(&input.to_ascii_lowercase())).collect()
            };


            for (idx, item) in filtered_items
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
                Key::Char(chr) => {
                    input.push(chr)
                },
                Key::Backspace => {
                  input.pop();
                },
                _ => {}
            }
            if sel != !0 && (sel < page * capacity || sel >= (page + 1) * capacity) {
                page = sel / capacity;
            }
            render.clear_preserve_prompt(&size_vec)?;
        }
    }
}