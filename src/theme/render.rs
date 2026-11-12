use std::{fmt, io};

use console::{measure_text_width, strip_ansi_codes, Term};
#[cfg(feature = "fuzzy-select")]
use fuzzy_matcher::skim::SkimMatcherV2;
use unicode_segmentation::UnicodeSegmentation;

use crate::{theme::Theme, Result};

/// Counts the terminal rows occupied by text that is followed by a newline.
///
/// Each explicit line occupies at least one row, and lines wider than the
/// terminal occupy additional rows through terminal wrapping. Wide grapheme
/// clusters wrap as a unit when they cannot fit in the remaining columns.
fn visual_line_count(text: &str, terminal_width: usize) -> usize {
    let terminal_width = terminal_width.max(1);
    let text = strip_ansi_codes(text);

    text.split('\n')
        .map(|line| {
            let mut rows = 1;
            let mut column = 0;

            for grapheme in line.graphemes(true) {
                let width = measure_text_width(grapheme);
                if width == 0 {
                    continue;
                }
                if column > 0 && column + width > terminal_width {
                    rows += 1;
                    column = 0;
                }
                column += width;
            }

            rows
        })
        .sum()
}

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
        self.height += visual_line_count(&buf, self.term.size().1 as usize);
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

    pub fn clear_preserve_prompt(&mut self) -> Result {
        self.term.clear_last_lines(self.height)?;
        self.height = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{visual_line_count, TermThemeRenderer};
    use crate::theme::SimpleTheme;
    use console::Term;

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

    #[test]
    fn visual_line_count_counts_wrapped_rows() {
        assert_eq!(visual_line_count("", 10), 1);
        assert_eq!(visual_line_count("1234567890", 10), 1);
        assert_eq!(visual_line_count("12345678901", 10), 2);
        assert_eq!(visual_line_count("123456789012345678901", 10), 3);
    }

    #[test]
    fn visual_line_count_includes_item_prefix_when_wrapping() {
        // The text itself fits, but the MultiSelect marker makes the rendered
        // item wrap onto a second terminal row.
        assert_eq!(visual_line_count("> [ ] 12345", 10), 2);
    }

    #[test]
    fn visual_line_count_handles_explicit_lines_and_display_width() {
        assert_eq!(visual_line_count("123456\n1234", 5), 3);
        assert_eq!(visual_line_count("界界界", 4), 2);
    }

    #[test]
    fn visual_line_count_wraps_wide_characters_before_the_right_edge() {
        // The wide character cannot fit in the one remaining column. Its
        // unused cell means these 20 display columns occupy three rows.
        assert_eq!(visual_line_count("> 1234567界123456789", 10), 3);
        assert_eq!(visual_line_count("abc界abc", 4), 3);
        assert_eq!(visual_line_count("abc界abc\nabc界abc", 4), 6);
        assert_eq!(visual_line_count("\x1b[31mabc界abc\x1b[0m", 4), 3);
    }

    #[test]
    fn visual_line_count_keeps_grapheme_clusters_together() {
        assert_eq!(visual_line_count("123456789👩‍💻123456789", 10), 3);
        assert_eq!(visual_line_count("123456789e\u{301}", 10), 1);
        assert_eq!(visual_line_count("123456789e\x1b[31m\u{301}\x1b[0m", 10), 1);
    }

    #[test]
    fn visual_line_count_handles_zero_terminal_width() {
        assert_eq!(visual_line_count("abc", 0), 3);
    }
}
