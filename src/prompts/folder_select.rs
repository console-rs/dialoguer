use std::{io, ops::Rem};

use console::{Key, Term};

use crate::{
    theme::{render::TermThemeRenderer, SimpleTheme, Theme},
    Paging, Result,
};

#[derive(Clone)]
pub struct FolderSelect<'a> {
    default: usize,
    items: Vec<String>,
    prompt: Option<String>,
    report: bool,
    clear: bool,
    file: bool,
    theme: &'a dyn Theme,
    max_length: Option<usize>,
    current_folder: String,
}

impl Default for FolderSelect<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl FolderSelect<'static> {
    /// Creates a folder_select prompt with default theme.
    pub fn new() -> Self {
        Self::with_theme(&SimpleTheme)
    }
}

impl FolderSelect<'_> {
    /// Indicates whether select menu should be erased from the screen after interaction.
    ///
    /// The default is to clear the menu.
    pub fn clear(mut self, val: bool) -> Self {
        self.clear = val;
        self
    }

    /// Indicates whether select should show both files and folder.
    ///
    /// The default is to show only folders.
    pub fn file(mut self, val: bool) -> Self {
        self.file = val;
        self
    }

    /// Sets an optional max length for a page.
    ///
    /// Max length is disabled by None
    pub fn max_length(mut self, val: usize) -> Self {
        // Paging subtracts two from the capacity, paging does this to
        // make an offset for the page indicator. So to make sure that
        // we can show the intended amount of items we need to add two
        // to our value.
        self.max_length = Some(val + 2);
        self
    }

    /// Sets the select prompt.
    ///
    /// By default, when a prompt is set the system also prints out a confirmation after
    /// the selection. You can opt-out of this with [`report`](Self::report).
    pub fn with_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.into());
        self.report = true;
        self
    }

    /// Sets the starting folder
    ///
    pub fn folder<T: ToString>(mut self, folder: T) -> Self {
        self.current_folder = folder.to_string();
        self
    }

    /// Processes the current folder to populate the items list for selection.
    ///
    /// This function reads the contents of the current folder, categorizes them into directories and files,
    /// and formats them according to the selected theme. The items list is then sorted and returned.
    ///
    /// # Panics
    ///
    /// This function panics if it fails to read the current folder.
    fn process_folder(mut self) -> Self {
        self.items.clear();
        let current_folder = std::path::PathBuf::from(&self.current_folder);

        // Add current directory to the items list
        self.items.push(".".to_string());

        // Add parent directory to the items list if it exists
        let parent_folder = current_folder.parent();
        if let Some(_parent_folder) = parent_folder {
            self.items.push("..".to_string());
        }

        let mut directories_in_current_folder = vec![];
        let mut files_in_current_folder = vec![];

        // Read the contents of the current folder
        if let Ok(entries) = std::fs::read_dir(current_folder) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        let name = entry.file_name().to_string_lossy().to_string();

                        // Categorize items into directories and files
                        if metadata.is_dir() {
                            directories_in_current_folder
                                .push(self.theme.format_folder_select_item(&name));
                        } else {
                            files_in_current_folder.push(self.theme.format_file_select_item(&name));
                        }
                    }
                }
            }
        } else {
            panic!("Failed to read current folder");
        }

        // Sort the items
        directories_in_current_folder.sort();
        // Places the folders above files
        self.items.extend(directories_in_current_folder);

        if self.file {
            files_in_current_folder.sort();
            self.items.extend(files_in_current_folder);
        }

        self
    }

    /// Indicates whether to report the selected value after interaction.
    ///
    /// The default is to report the selection.
    pub fn report(mut self, val: bool) -> Self {
        self.report = val;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items with the 'Space' bar or 'Enter' and the index of selected item will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `index` if user selected one of items using 'Enter'.
    /// This unlike [`interact_opt`](Self::interact_opt) does not allow to quit with 'Esc' or 'q'.
    #[inline]
    pub fn interact(self) -> Result<String> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items with the 'Space' bar or 'Enter' and the index of selected item will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `Some(index)` if user selected one of items using 'Enter' or `None` if user cancelled with 'Esc' or 'q'.
    ///
    /// ## Example
    ///
    ///```rust,no_run
    /// use dialoguer::Select;
    ///
    /// fn main() {
    ///     let items = vec!["foo", "bar", "baz"];
    ///
    ///     let selection = Select::new()
    ///         .with_prompt("What do you choose?")
    ///         .items(&items)
    ///         .interact_opt()
    ///         .unwrap();
    ///
    ///     match selection {
    ///         Some(index) => println!("You chose: {}", items[index]),
    ///         None => println!("You did not choose anything.")
    ///     }
    /// }
    ///```
    #[inline]
    pub fn interact_opt(self) -> Result<Option<String>> {
        self.interact_on_opt(&Term::stderr())
    }

    /// Like [`interact`](Self::interact) but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on(self, term: &Term) -> Result<String> {
        Ok(self
            ._interact_on(term, false)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case"))?)
    }

    /// Like [`interact_opt`](Self::interact_opt) but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on_opt(self, term: &Term) -> Result<Option<String>> {
        self._interact_on(term, true)
    }

    /// Like `interact` but allows a specific terminal to be set.
    fn _interact_on(mut self, term: &Term, allow_quit: bool) -> Result<Option<String>> {
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not a terminal").into());
        }

        self = self.process_folder();

        if self.items.is_empty() {
            // this should never happen and could be removed
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Empty list of items given to `Select`",
            ))?;
        }

        let mut paging = Paging::new(term, self.items.len(), self.max_length);
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = self.default;

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

        term.hide_cursor()?;
        paging.update_page(sel);

        loop {
            if let Some(ref prompt) = self.prompt {
                paging.render_prompt(|paging_info| render.select_prompt(prompt, paging_info))?;
            }
            render.folder_select_path(&format!("Current folder: {}", self.current_folder))?; //TODO: parametrize message

            for (idx, item) in self
                .items
                .iter()
                .enumerate()
                .skip(paging.current_page * paging.capacity)
                .take(paging.capacity)
            {
                render.select_prompt_item(item, sel == idx)?;
            }

            term.flush()?;

            match term.read_key()? {
                Key::ArrowDown | Key::Tab | Key::Char('j') => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(self.items.len() as u64) as usize;
                    }
                }
                Key::Escape | Key::Char('q') => {
                    if allow_quit {
                        if self.clear {
                            render.clear()?;
                        } else {
                            term.clear_last_lines(paging.capacity)?;
                        }

                        term.show_cursor()?;
                        term.flush()?;

                        return Ok(None);
                    }
                }
                Key::ArrowUp | Key::BackTab | Key::Char('k') => {
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

                Key::Enter | Key::Char(' ') if sel != !0 => {
                    if self.items[sel] == "." {
                        if self.clear {
                            render.clear()?;
                        }

                        if let Some(ref prompt) = self.prompt {
                            if self.report {
                                render.select_prompt_selection(prompt, &self.items[sel])?;
                            }
                        }

                        term.show_cursor()?;
                        term.flush()?;

                        return Ok(Some(self.current_folder));
                    } else if self.items[sel] == ".." {
                        let p = std::path::PathBuf::from(&self.current_folder)
                            .parent()
                            .unwrap()
                            .to_string_lossy()
                            .to_string();
                        self.current_folder = p;
                        self = self.process_folder();
                    } else {
                        let selection = match self.items[sel].find(' ') {
                            Some(pos) => &self.items[sel][pos + 1..],
                            None => &self.items[sel],
                        };
                        let mut p = std::path::PathBuf::from(&self.current_folder);
                        p.push(std::path::Path::new(selection));
                        let selected_path_name = p.to_string_lossy().to_string();

                        match std::fs::metadata(p) {
                            Ok(metadata) if metadata.is_dir() => {
                                self.current_folder = selected_path_name;
                                self = self.process_folder();
                            }
                            Ok(metadata) if metadata.is_file() => {
                                if self.clear {
                                    render.clear()?;
                                }

                                if let Some(ref prompt) = self.prompt {
                                    if self.report {
                                        render.select_prompt_selection(prompt, &self.items[sel])?;
                                    }
                                }

                                term.show_cursor()?;
                                term.flush()?;

                                return Ok(Some(selected_path_name));
                            }
                            _ => {
                                return Ok(None); // probably return error
                            }
                        }
                    }
                    // return Ok(Some(sel));
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

impl<'a> FolderSelect<'a> {
    /// Creates a files select prompt with a specific theme.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::{theme::ColorfulTheme, Select};
    ///
    /// fn main() {
    ///     let selection = FolderSelect::with_theme(&ColorfulTheme::default())
    ///           .with_prompt("Select some file from /tmp")
    ///           .folder("/tmp")
    ///           .file(true)
    ///           .interact()
    ///           .unwrap();
    /// }
    /// ```
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            default: !0,
            items: vec![],
            prompt: None,
            report: false,
            clear: true,
            file: false,
            max_length: None,
            theme,
            current_folder: ".".to_string(),
        }
    }
}
