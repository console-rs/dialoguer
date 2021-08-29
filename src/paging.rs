use std::io;

use console::Term;

/// Creates a paging module
///
/// The paging module serves as tracking structure to allow paged views
/// and automatically (de-)activates paging depending on the current terminal size.
pub struct Paging<'a> {
    term: &'a Term,
    current_term_size: (u16, u16),
    pages: usize,
    current_page: usize,
    capacity: usize,
    items_len: usize,
    active: bool,
    activity_transition: bool,
}

impl<'a> Paging<'a> {
    pub fn new(term: &'a Term, items_len: usize) -> Paging<'a> {
        let capacity = term.size().0 as usize - 2;
        let pages = (items_len as f64 / capacity as f64).ceil() as usize;

        Paging {
            term,
            current_term_size: term.size(),
            pages,
            current_page: 0,
            capacity,
            items_len,
            active: pages > 1,
            activity_transition: true,
        }
    }

    /// Updates all internal based on the current terminal size and cursor position
    pub fn update(&mut self, cursor_pos: usize) -> io::Result<()> {
        if self.current_term_size != self.term.size() {
            self.current_term_size = self.term.size();
            self.capacity = self.current_term_size.0 as usize - 2;
            self.pages = (self.items_len as f64 / self.capacity as f64).ceil() as usize;
        }

        if self.active != (self.pages > 1) {
            self.active = self.pages > 1;
            self.activity_transition = true;
            self.term.clear_last_lines(self.capacity)?;
        } else {
            self.activity_transition = false;
        }

        if cursor_pos != !0
            && (cursor_pos < self.current_page * self.capacity
                || cursor_pos >= (self.current_page + 1) * self.capacity)
        {
            self.current_page = cursor_pos / self.capacity;
        }

        Ok(())
    }

    /// Returns current acivity state
    pub fn active(&self) -> bool {
        self.active
    }

    /// Returns capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns number of pages
    pub fn pages(&self) -> usize {
        self.pages
    }

    /// Returns current page
    pub fn current_page(&self) -> usize {
        self.current_page
    }

    /// Renders a prompt for paging when the following conditions are met:
    /// * Paging is active
    /// * Transition of the paging activity happened (active -> inactive / inactive -> active)
    pub fn render_prompt<F: FnMut(Option<(usize, usize)>) -> io::Result<()>>(
        &mut self,
        mut render_prompt: F,
    ) -> io::Result<()> {
        let mut paging_info = None;

        if self.active {
            paging_info = Some((self.current_page + 1, self.pages));
            render_prompt(paging_info)?;
        } else if self.activity_transition {
            render_prompt(paging_info)?;
        }

        self.term.flush()?;

        Ok(())
    }

    /// Navigates to the next page
    pub fn next_page(&mut self) -> usize {
        if self.current_page == self.pages - 1 {
            self.current_page = 0;
        } else {
            self.current_page += 1;
        }

        self.current_page * self.capacity
    }

    /// Navigates to the previous page
    pub fn previous_page(&mut self) -> usize {
        if self.current_page == 0 {
            self.current_page = self.pages - 1;
        } else {
            self.current_page -= 1;
        }

        self.current_page * self.capacity
    }
}
