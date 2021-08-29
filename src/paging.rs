use std::io;

use console::Term;

pub struct Paging<'a> {
    term: &'a Term,
    current_term_size: (u16, u16),
    pages: usize,
    current_page: usize,
    capacity: usize,
    items: &'a Vec<String>,
    active: bool,
    activity_transition: bool,
}

impl<'a> Paging<'a> {
    pub fn new(term: &'a Term, items: &'a Vec<String>) -> Paging<'a> {
        let capacity = term.size().0 as usize - 2;
        let pages = (items.len() as f64 / capacity as f64).ceil() as usize;

        Paging {
            term,
            current_term_size: term.size(),
            pages,
            current_page: 0,
            capacity,
            items,
            active: pages > 1,
            activity_transition: true,
        }
    }

    pub fn update(&mut self, sel: usize) -> io::Result<()> {
        if self.current_term_size != self.term.size() {
            self.current_term_size = self.term.size();
            self.capacity = self.current_term_size.0 as usize - 2;
            self.pages = (self.items.len() as f64 / self.capacity as f64).ceil() as usize;
        }

        if self.active != (self.pages > 1) {
            self.active = self.pages > 1;
            self.activity_transition = true;
            self.term.clear_last_lines(self.capacity)?;
        } else {
            self.activity_transition = false;
        }

        if sel != !0
            && (sel < self.current_page * self.capacity
                || sel >= (self.current_page + 1) * self.capacity)
        {
            self.current_page = sel / self.capacity;
        }

        Ok(())
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn pages(&self) -> usize {
        self.pages
    }

    pub fn current_page(&self) -> usize {
        self.current_page
    }

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

    pub fn next_page(&mut self) -> usize {
        if self.current_page == self.pages - 1 {
            self.current_page = 0;
        } else {
            self.current_page += 1;
        }

        self.current_page * self.capacity
    }

    pub fn previous_page(&mut self) -> usize {
        if self.current_page == 0 {
            self.current_page = self.pages - 1;
        } else {
            self.current_page -= 1;
        }

        self.current_page * self.capacity
    }
}
