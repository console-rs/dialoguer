use std::io;

use console::{Key, Term};

use crate::{
    theme::{render::TermThemeRenderer, SimpleTheme, Theme},
    Result,
};

/// Renders an alert prompt.
///
/// ## Example
///
/// ```rust,no_run
/// use dialoguer::{theme::ColorfulTheme, Alert};
///
/// fn main() {
///     let _ = Alert::with_theme(&ColorfulTheme::default())
///         .with_prompt("Something went wrong!  Press enter to continue.")
///         .interact();
///
///     let _ = Alert::with_theme(&ColorfulTheme::default())
///         .with_alert_text("This is an alert, press enter to continue.")
///         .interact();
///
///     let _ = Alert::with_theme(&ColorfulTheme::default())
///         .with_alert_text("Strange things happened: <spooky error message>.")
///         .with_prompt("Press enter to continue.")
///         .interact();
/// }
/// ```
#[derive(Clone)]
pub struct Alert<'a> {
    alert_text: String,
    prompt: String,
    theme: &'a dyn Theme,
}

impl Default for Alert<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl Alert<'static> {
    /// Creates a alert prompt with default theme.
    pub fn new() -> Self {
        Self::with_theme(&SimpleTheme)
    }
}

impl Alert<'_> {
    /// Sets the alert content message.
    pub fn with_alert_text<S: Into<String>>(mut self, alert_text: S) -> Self {
        self.alert_text = alert_text.into();
        self
    }

    /// Sets the alert prompt.
    pub fn with_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = prompt.into();
        self
    }

    /// Enables user interaction.
    ///
    /// The dialog is rendered on stderr.
    #[inline]
    pub fn interact(self) -> Result<Option<()>> {
        self.interact_on(&Term::stderr())
    }

    /// Like [`interact`](Self::interact) but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on(self, term: &Term) -> Result<Option<()>> {
        Ok(Some(self._interact_on(term)?.ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case")
        })?))
    }

    fn _interact_on(self, term: &Term) -> Result<Option<()>> {
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not a terminal").into());
        }

        let mut render = TermThemeRenderer::new(term, self.theme);

        render.alert_prompt(&self.alert_text, &self.prompt)?;

        term.hide_cursor()?;
        term.flush()?;

        // Default behavior:  wait for user to hit the Enter key.
        loop {
            let input = term.read_key()?;
            match input {
                Key::Enter => (),
                _ => {
                    continue;
                }
            };

            break;
        }

        term.write_line("")?;
        term.show_cursor()?;
        term.flush()?;

        Ok(Some(()))
    }
}

impl<'a> Alert<'a> {
    /// Creates an alert prompt with a specific theme.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::{theme::ColorfulTheme, Alert};
    ///
    /// fn main() {
    ///     let alert = Alert::with_theme(&ColorfulTheme::default())
    ///         .interact();
    /// }
    /// ```
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            alert_text: "".into(),
            prompt: "".into(),
            theme,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let alert = Alert::new()
            .with_alert_text("FYI: ground gets wet if it rains.")
            .with_prompt("Press enter continue");

        let _ = alert.clone();
    }
}
