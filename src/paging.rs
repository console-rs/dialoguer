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
}

impl<'a> Paging<'a> {
    pub fn new(term: &'a Term, items: &'a Vec<String>) -> Paging<'a> {
        // Substract 2, because we show the prompt and/or page info on every page
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
        }
    }

    pub fn update(&mut self, sel: usize) -> io::Result<()> {
        if self.current_term_size != self.term.size() {
            self.current_term_size = self.term.size();
            self.capacity = self.current_term_size.0 as usize - 2;
            self.pages = (self.items.len() as f64 / self.capacity as f64).ceil() as usize;
            self.active = self.pages > 1;
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

    pub fn render_page_items<F: FnMut(usize, &String) -> io::Result<()>>(
        &self,
        mut render_item: F,
    ) -> io::Result<()> {
        for (idx, item) in self
            .items
            .iter()
            .enumerate()
            .skip(self.current_page * self.capacity)
            .take(self.capacity)
        {
            render_item(idx, item).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
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
