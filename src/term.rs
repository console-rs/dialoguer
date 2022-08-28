use std::{
    io,
    sync::{Arc, Mutex},
};

use crossterm::{cursor, ExecutableCommand};

/// Wrapper around a terminal: anything containing a Write instance.
///
/// Cloning this type is rather cheap: it's just cloning an `Arc`.
#[derive(Clone)]
pub struct Term {
    inner: Arc<Mutex<dyn io::Write>>,
}

impl Term {
    pub(crate) fn new(data: Arc<Mutex<dyn io::Write>>) -> Self {
        Term { inner: data }
    }

    /// Execute a command on the underlying terminal.
    pub(crate) fn execute(&self, cmd: impl crossterm::Command) -> io::Result<&Self> {
        self.inner.lock().unwrap().execute(cmd)?;
        Ok(self)
    }

    /// Attempt to write an entire buffer into this terminal.
    pub(crate) fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        self.inner.lock().unwrap().write_all(buf)
    }

    /// Flush the underlying terminal.
    pub(crate) fn flush(&self) -> io::Result<()> {
        self.inner.lock().unwrap().flush()?;
        Ok(())
    }

    /// Hide the cursor on the underlying terminal.
    pub(crate) fn hide_cursor(&self) -> io::Result<()> {
        self.inner.lock().unwrap().execute(cursor::Hide)?;
        Ok(())
    }

    /// Show the cursor on the underlying terminal.
    pub(crate) fn show_cursor(&self) -> io::Result<()> {
        self.inner.lock().unwrap().execute(cursor::Show)?;
        Ok(())
    }
}
