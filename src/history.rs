use std::collections::VecDeque;

/// Trait for history handling.
pub trait History<T> {
    /// This is called with the current position that should
    /// be read from history. The `pos` represents the number
    /// of times the `Up`/`Down` arrow key has been pressed.
    /// This would normally be used as an index to some sort
    /// of vector.  If the `pos` does not have an entry, [`None`](Option::None)
    /// should be returned.
    fn read(&self, pos: usize) -> Option<String>;

    /// This is called with the next value you should store
    /// in history at the first location. Normally history
    /// is implemented as a FIFO queue.
    fn write(&mut self, val: &T);
}

impl History<String> for Vec<String> {
    fn read(&self, pos: usize) -> Option<String> {
        // We have to check manually here instead of using `Vec::get` since
        // subtracting from `usize` into the negative throws an exception.
        if pos >= self.len() {
            None
        } else {
            // Since we have already ensured that `pos`
            // is in bounds, we can use direct access.
            Some(self[self.len() - pos - 1].clone())
        }
    }

    fn write(&mut self, val: &String) {
        self.push(val.clone())
    }
}

impl History<String> for VecDeque<String> {
    fn read(&self, pos: usize) -> Option<String> {
        self.get(pos).cloned()
    }

    fn write(&mut self, val: &String) {
        // With `VecDeque` we can simply use `push_front`,
        // allowing for normal forward indexing in `read`.
        self.push_front(val.clone())
    }
}
