use std::fmt;
use std::io;

use console::{style, Term};

#[derive(Debug, Clone, Copy)]
pub enum SelectionStyle {
    CheckboxUncheckedSelected,
    CheckboxUncheckedUnselected,
    CheckboxCheckedSelected,
    CheckboxCheckedUnselected,
    MenuSelected,
    MenuUnselected,
}

/// Implements a theme for dialoguer.
pub trait Theme {
    /// Given a prompt this formats out what the prompt should look like.
    fn format_prompt(&self, f: &mut fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}:", prompt)
    }

    /// Renders a prompt and a single selection made.
    fn format_single_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}: {}", prompt, sel)
    }

    /// Renders a prompt and multiple selections,
    fn format_multi_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
        }
        Ok(())
    }

    /// Formats a selection.
    fn format_selection(
        &self,
        f: &mut fmt::Write,
        text: &str,
        style: SelectionStyle,
    ) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match style {
                SelectionStyle::CheckboxUncheckedSelected => "> [ ] ",
                SelectionStyle::CheckboxUncheckedUnselected => "  [ ] ",
                SelectionStyle::CheckboxCheckedSelected => "> [x] ",
                SelectionStyle::CheckboxCheckedUnselected => "  [x] ",
                SelectionStyle::MenuSelected => "> ",
                SelectionStyle::MenuUnselected => "  ",
            },
            text
        )
    }
}

/// The default theme.
pub struct DefaultTheme;

impl Theme for DefaultTheme {}

/// A colorful theme
pub struct ColorfulTheme;

impl Theme for ColorfulTheme {
    fn format_prompt(&self, f: &mut fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}:", prompt)
    }

    fn format_single_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}: {}", prompt, style(sel).cyan())
    }

    fn format_multi_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(
                f,
                "{}{}",
                if idx == 0 { "" } else { ", " },
                style(sel).cyan()
            )?;
        }
        Ok(())
    }

    fn format_selection(&self, f: &mut fmt::Write, text: &str, st: SelectionStyle) -> fmt::Result {
        match st {
            SelectionStyle::CheckboxUncheckedSelected => {
                write!(f, "{} [ ] {}", style(">").cyan().bold(), text)
            }
            SelectionStyle::CheckboxUncheckedUnselected => write!(f, "  [ ] {}", style(text).dim()),
            SelectionStyle::CheckboxCheckedSelected => write!(
                f,
                "{} [{}] {}",
                style(">").cyan().bold(),
                style("x").green().bold(),
                text
            ),
            SelectionStyle::CheckboxCheckedUnselected => {
                write!(f, "  [{}] {}", style("x").green().bold(), style(text).dim())
            }
            SelectionStyle::MenuSelected => write!(f, "{} {}", style(">").cyan().bold(), text),
            SelectionStyle::MenuUnselected => write!(f, "  {}", style(text).dim()),
        }
    }
}

/// Helper struct to conveniently render a theme ot a term.
pub struct TermThemeRenderer<'a> {
    term: &'a Term,
    theme: &'a Theme,
    height: usize,
    prompt_height: usize,
}

impl<'a> TermThemeRenderer<'a> {
    pub fn new(term: &'a Term, theme: &'a Theme) -> TermThemeRenderer<'a> {
        TermThemeRenderer {
            term: term,
            theme: theme,
            height: 0,
            prompt_height: 0,
        }
    }

    fn write_formatted_line<F: FnOnce(&mut TermThemeRenderer, &mut fmt::Write) -> fmt::Result>(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count() + 1;
        self.term.write_line(&buf)
    }

    pub fn prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_line(|this, buf| this.theme.format_prompt(buf, prompt))?;
        self.prompt_height = self.height;
        self.height = 0;
        Ok(())
    }

    pub fn single_prompt_selection(&mut self, prompt: &str, sel: &str) -> io::Result<()> {
        self.write_formatted_line(|this, buf| {
            this.theme.format_single_prompt_selection(buf, prompt, sel)
        })?;
        self.prompt_height = self.height;
        self.height = 0;
        Ok(())
    }

    pub fn multi_prompt_selection(&mut self, prompt: &str, selections: &[&str]) -> io::Result<()> {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_multi_prompt_selection(buf, prompt, selections)
        })?;
        self.prompt_height = self.height;
        self.height = 0;
        Ok(())
    }

    pub fn selection(&mut self, text: &str, style: SelectionStyle) -> io::Result<()> {
        self.write_formatted_line(|this, buf| this.theme.format_selection(buf, text, style))
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.term
            .clear_last_lines(self.height + self.prompt_height)?;
        self.height = 0;
        Ok(())
    }

    pub fn clear_preserve_prompt(&mut self) -> io::Result<()> {
        self.term.clear_last_lines(self.height)?;
        self.height = 0;
        Ok(())
    }
}
