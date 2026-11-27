use std::{fmt, io};

use console::{measure_text_width, Term};
#[cfg(feature = "fuzzy-select")]
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::{theme::Theme, MultiSelectPlusItem, Result};

/// Helper struct to conveniently render a theme.
pub(crate) struct TermThemeRenderer<'a> {
    term: &'a Term,
    theme: &'a dyn Theme,
    height: usize,
    prompt_height: usize,
    prompts_reset_height: bool,
}

impl<'a> TermThemeRenderer<'a> {
    pub fn new(term: &'a Term, theme: &'a dyn Theme) -> TermThemeRenderer<'a> {
        TermThemeRenderer {
            term,
            theme,
            height: 0,
            prompt_height: 0,
            prompts_reset_height: true,
        }
    }

    #[cfg(feature = "password")]
    pub fn set_prompts_reset_height(&mut self, val: bool) {
        self.prompts_reset_height = val;
    }

    #[cfg(feature = "password")]
    pub fn term(&self) -> &Term {
        self.term
    }

    pub fn add_line(&mut self) {
        self.height += 1;
    }

    fn write_formatted_str<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> Result<usize> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(io::Error::other)?;
        self.height += buf.chars().filter(|&x| x == '\n').count();
        self.term.write_str(&buf)?;
        Ok(measure_text_width(
            buf.rsplit('\n').next().unwrap_or_default(),
        ))
    }

    fn write_formatted_line<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> Result {
        let mut buf = String::new();
        f(self, &mut buf).map_err(io::Error::other)?;
        self.height += buf.chars().filter(|&x| x == '\n').count() + 1;
        Ok(self.term.write_line(&buf)?)
    }

    fn write_formatted_prompt<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> Result {
        self.write_formatted_line(f)?;
        if self.prompts_reset_height {
            self.prompt_height = self.height;
            self.height = 0;
        }
        Ok(())
    }

    fn write_paging_info(buf: &mut dyn fmt::Write, paging_info: (usize, usize)) -> fmt::Result {
        write!(buf, " [Page {}/{}] ", paging_info.0, paging_info.1)
    }

    pub fn error(&mut self, err: &str) -> Result {
        self.write_formatted_line(|this, buf| this.theme.format_error(buf, err))
    }

    pub fn confirm_prompt(&mut self, prompt: &str, default: Option<bool>) -> Result<usize> {
        self.write_formatted_str(|this, buf| this.theme.format_confirm_prompt(buf, prompt, default))
    }

    pub fn confirm_prompt_no_hint(&mut self, prompt: &str) -> Result<usize> {
        self.write_formatted_str(|this, buf| this.theme.format_confirm_prompt_no_hint(buf, prompt))
    }

    pub fn confirm_prompt_selection(&mut self, prompt: &str, sel: Option<bool>) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_confirm_prompt_selection(buf, prompt, sel)
        })
    }

    #[cfg(feature = "fuzzy-select")]
    pub fn fuzzy_select_prompt(
        &mut self,
        prompt: &str,
        search_term: &str,
        cursor_pos: usize,
    ) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_fuzzy_select_prompt(buf, prompt, search_term, cursor_pos)
        })
    }

    pub fn input_prompt(&mut self, prompt: &str, default: Option<&str>) -> Result<usize> {
        self.write_formatted_str(|this, buf| this.theme.format_input_prompt(buf, prompt, default))
    }

    pub fn input_prompt_selection(&mut self, prompt: &str, sel: &str) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_input_prompt_selection(buf, prompt, sel)
        })
    }

    #[cfg(feature = "password")]
    pub fn password_prompt(&mut self, prompt: &str) -> Result<usize> {
        self.write_formatted_str(|this, buf| {
            write!(buf, "\r")?;
            this.theme.format_password_prompt(buf, prompt)
        })
    }

    #[cfg(feature = "password")]
    pub fn password_prompt_selection(&mut self, prompt: &str) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_password_prompt_selection(buf, prompt)
        })
    }

    pub fn select_prompt(&mut self, prompt: &str, paging_info: Option<(usize, usize)>) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_select_prompt(buf, prompt)?;

            if let Some(paging_info) = paging_info {
                TermThemeRenderer::write_paging_info(buf, paging_info)?;
            }

            Ok(())
        })
    }

    pub fn select_prompt_selection(&mut self, prompt: &str, sel: &str) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_select_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn select_prompt_item(&mut self, text: &str, active: bool) -> Result {
        self.write_formatted_line(|this, buf| {
            this.theme.format_select_prompt_item(buf, text, active)
        })
    }

    #[cfg(feature = "fuzzy-select")]
    pub fn fuzzy_select_prompt_item(
        &mut self,
        text: &str,
        active: bool,
        highlight: bool,
        matcher: &SkimMatcherV2,
        search_term: &str,
    ) -> Result {
        self.write_formatted_line(|this, buf| {
            this.theme.format_fuzzy_select_prompt_item(
                buf,
                text,
                active,
                highlight,
                matcher,
                search_term,
            )
        })
    }

    pub fn multi_select_prompt(
        &mut self,
        prompt: &str,
        paging_info: Option<(usize, usize)>,
    ) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_multi_select_prompt(buf, prompt)?;

            if let Some(paging_info) = paging_info {
                TermThemeRenderer::write_paging_info(buf, paging_info)?;
            }

            Ok(())
        })
    }

    pub fn multi_select_prompt_selection(&mut self, prompt: &str, sel: &[&str]) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_multi_select_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn multi_select_prompt_item(&mut self, text: &str, checked: bool, active: bool) -> Result {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_multi_select_prompt_item(buf, text, checked, active)
        })
    }

    pub fn multi_select_plus_prompt_item(
        &mut self,
        item: &MultiSelectPlusItem,
        active: bool,
    ) -> Result {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_multi_select_plus_prompt_item(buf, item, active)
        })
    }

    pub fn sort_prompt(&mut self, prompt: &str, paging_info: Option<(usize, usize)>) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_sort_prompt(buf, prompt)?;

            if let Some(paging_info) = paging_info {
                TermThemeRenderer::write_paging_info(buf, paging_info)?;
            }

            Ok(())
        })
    }

    pub fn sort_prompt_selection(&mut self, prompt: &str, sel: &[&str]) -> Result {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_sort_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn sort_prompt_item(&mut self, text: &str, picked: bool, active: bool) -> Result {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_sort_prompt_item(buf, text, picked, active)
        })
    }

    pub fn clear(&mut self) -> Result {
        self.term
            .clear_last_lines(self.height + self.prompt_height)?;
        self.height = 0;
        self.prompt_height = 0;
        Ok(())
    }

    pub fn clear_preserve_prompt(&mut self, size_vec: &[usize]) -> Result {
        let prefix_width = 2;
        let new_height =
            self.height + overflow_lines(size_vec, self.term.size().1 as usize, prefix_width);

        self.term.clear_last_lines(new_height)?;
        self.height = 0;
        Ok(())
    }
}

/// Number of extra visual lines taken up by items that wrap because they are wider than
/// the terminal. `size_vec` holds the display width of each rendered item, and every item
/// is prefixed by `prefix_width` columns.
fn overflow_lines(size_vec: &[usize], term_width: usize, prefix_width: usize) -> usize {
    if term_width == 0 {
        return 0;
    }

    size_vec
        .iter()
        .map(|&size| {
            let total = size.saturating_add(prefix_width);
            total.div_ceil(term_width).saturating_sub(1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{overflow_lines, TermThemeRenderer};
    use crate::theme::SimpleTheme;
    use console::Term;

    #[test]
    fn overflow_lines_accounts_for_the_item_prefix() {
        // An item that is exactly as wide as the terminal fits on one line, but the
        // two-column prefix pushes anything wider over the edge.
        assert_eq!(overflow_lines(&[38], 40, 2), 0);
        assert_eq!(overflow_lines(&[39], 40, 2), 1);
        assert_eq!(overflow_lines(&[100], 40, 2), 2);
        assert_eq!(overflow_lines(&[100, 100], 40, 2), 4);
    }

    #[test]
    fn overflow_lines_without_a_known_width() {
        assert_eq!(overflow_lines(&[100], 0, 2), 0);
    }

    #[test]
    fn only_rendered_items_contribute_overflow_lines() {
        // A fuzzy select may hold many items but only render the visible window.
        // Off-screen wide items must not inflate the number of lines to clear.
        let all_items = [200, 200, 200];
        let rendered_items = [200];
        assert!(
            overflow_lines(&all_items, 40, 2) > overflow_lines(&rendered_items, 40, 2),
            "all items would clear more lines than the rendered window"
        );
    }

    #[test]
    fn input_prompt_width_uses_last_line() {
        let term = Term::buffered_stderr();
        let mut render = TermThemeRenderer::new(&term, &SimpleTheme);

        for (prompt, default, expected) in [
            ("prompt", None, 8),
            ("first\nx", None, 3),
            ("first\n", None, 2),
            ("first\n\x1b[31m界\x1b[0m", None, 4),
            ("prompt", Some("first\nlast"), 7),
        ] {
            assert_eq!(render.input_prompt(prompt, default).unwrap(), expected);
        }
    }
}
