//! Column alignment for selection prompts.
//!
//! When options carry several data fields, rendering them as plain text makes
//! the values hard to compare and scan. Configuring columns aligns those fields
//! so that related values line up underneath each other.
//!
//! Each item is a single string whose columns are delimited by the separators
//! declared in the [`ColumnConfig`] list. The list holds one entry per column
//! boundary: `N` configs describe `N + 1` columns, and the final column is the
//! remainder of the string. Every item is split left-to-right on those exact
//! separators, the cells are padded according to each column's alignment, and
//! the row is re-joined with the same separators.
//!
//! Because splitting happens on the first occurrence of each separator, a cell
//! must not contain its own column's separator; otherwise the split lands
//! inside the value. The row still aligns, but the column boundary moves (for
//! example `898.95 KB` split on `" "` yields `898.95` and `KB ...`).
//!
//! ## Example
//!
//! ```rust,no_run
//! use dialoguer::{
//!     tabular::{ColumnAlignment, ColumnConfig},
//!     MultiSelect,
//! };
//!
//! fn main() {
//!     let items = vec![
//!         "copy_current_location: 898.95 KB (2025-10-12 15:41), /path1",
//!         "summary-gen: 211.29 MB (2025-10-13 20:04), /path2",
//!         "rona: 1.26 GB (2025-10-14 18:29), /path3",
//!     ];
//!
//!     let columns = vec![
//!         ColumnConfig::new_with_separator(": ", ColumnAlignment::Left),
//!         ColumnConfig::new(ColumnAlignment::Left), // default " " separator
//!         ColumnConfig::new_with_separator(", ", ColumnAlignment::Left),
//!     ];
//!
//!     let selection = MultiSelect::new()
//!         .with_prompt("Select projects to clean")
//!         .items(&items)
//!         .with_tabular_columns(columns)
//!         .interact()
//!         .unwrap();
//! }
//! ```

use console::measure_text_width;

/// Horizontal alignment of a column's cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAlignment {
    /// Cells are aligned to the left, padding is added on the right.
    Left,
    /// Cells are aligned to the right, padding is added on the left.
    Right,
}

/// Where a left-aligned column's separator sits relative to its padding.
///
/// This only affects [`ColumnAlignment::Left`] columns (a right-aligned column
/// already keeps its padding on the left, so the separator always hugs the
/// content).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorPosition {
    /// The separator immediately follows the cell content and padding trails it.
    ///
    /// Good for label-like separators that belong to the value, e.g. `name:`.
    /// This is the default.
    AfterContent,
    /// The separator is placed just before the next column, after the padding.
    ///
    /// Good for divider-like separators such as `" | "`, so they line up into a
    /// visible column of their own.
    BeforeNextColumn,
}

/// Configuration for a single column boundary.
///
/// A column is defined by its [`ColumnAlignment`], the separator rendered after
/// its content, and the delimiter used to split it off the incoming string.
///
/// The **separator** is what appears in the output between this column and the
/// next (defaulting to a single space); alignment padding is added around the
/// content so the following column lines up. The **delimiter** is what the
/// incoming string is split on to recover this column's cell; it defaults to
/// the separator.
///
/// Splitting the input and rendering the output are therefore independent: you
/// can parse space-separated data yet render it with a wider, more legible
/// separator such as `" | "` or several spaces.
///
/// ```rust
/// # use dialoguer::tabular::{ColumnAlignment, ColumnConfig};
/// // Split the input on a single space, but render columns separated by " | ".
/// let column = ColumnConfig::new_with_separator(" | ", ColumnAlignment::Left).delimiter(" ");
/// ```
#[derive(Debug, Clone)]
pub struct ColumnConfig {
    separator: String,
    delimiter: Option<String>,
    alignment: ColumnAlignment,
    separator_position: SeparatorPosition,
}

impl ColumnConfig {
    /// Creates a column configuration with the default separator (a single space).
    ///
    /// Equivalent to `ColumnConfig::new_with_separator(" ", alignment)`.
    pub fn new(alignment: ColumnAlignment) -> Self {
        Self::new_with_separator(" ", alignment)
    }

    /// Creates a column configuration with a custom separator.
    ///
    /// The separator is used both to split the input and to render the output
    /// until a distinct [`delimiter`](Self::delimiter) is set.
    pub fn new_with_separator<S: Into<String>>(separator: S, alignment: ColumnAlignment) -> Self {
        Self {
            separator: separator.into(),
            delimiter: None,
            alignment,
            separator_position: SeparatorPosition::AfterContent,
        }
    }

    /// Sets (or replaces) the output separator using the builder pattern.
    pub fn separator<S: Into<String>>(mut self, separator: S) -> Self {
        self.separator = separator.into();
        self
    }

    /// Sets the delimiter used to split this column off the input string.
    ///
    /// By default the input is split on the [`separator`](Self::separator). Use
    /// this when the incoming data uses a different (usually narrower) delimiter
    /// than the separator you want to render, e.g. splitting on `" "` while
    /// rendering `" | "`.
    pub fn delimiter<S: Into<String>>(mut self, delimiter: S) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }

    /// Sets (or replaces) the alignment using the builder pattern.
    pub fn alignment(mut self, alignment: ColumnAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Sets where the separator sits relative to the column's padding.
    ///
    /// Defaults to [`SeparatorPosition::AfterContent`]. Use
    /// [`SeparatorPosition::BeforeNextColumn`] to line divider-like separators
    /// up into their own column. Only affects left-aligned columns.
    pub fn separator_position(mut self, position: SeparatorPosition) -> Self {
        self.separator_position = position;
        self
    }

    /// The token the input is split on: the delimiter if set, else the separator.
    fn split_token(&self) -> &str {
        self.delimiter.as_deref().unwrap_or(&self.separator)
    }
}

impl Default for ColumnConfig {
    fn default() -> Self {
        Self::new(ColumnAlignment::Left)
    }
}

/// Aligns `items` into columns according to `columns`.
///
/// Each item is split into `columns.len() + 1` cells on the configured
/// separators (in order, first occurrence each). Cells are then padded per
/// column so that related fields line up, and the row is re-joined with the
/// same separators. The final column is emitted verbatim.
pub(crate) fn format_rows(items: &[String], columns: &[ColumnConfig]) -> Vec<String> {
    // With no configured boundaries there is nothing to split or align.
    if columns.is_empty() {
        return items.to_vec();
    }

    // Split each item into cells: one cell per configured column plus a final
    // cell holding the remainder.
    let rows: Vec<Vec<&str>> = items
        .iter()
        .map(|item| {
            let mut cells = Vec::with_capacity(columns.len() + 1);
            let mut rest = item.as_str();

            for column in columns {
                match rest.split_once(column.split_token()) {
                    Some((cell, remainder)) => {
                        cells.push(cell);
                        rest = remainder;
                    }
                    None => {
                        // Separator missing: this cell takes the remainder and
                        // the trailing columns stay empty.
                        cells.push(rest);
                        rest = "";
                    }
                }
            }

            cells.push(rest);
            cells
        })
        .collect();

    // Content width (excluding the separator) of every non-final column.
    let widths: Vec<usize> = (0..columns.len())
        .map(|c| {
            rows.iter()
                .map(|row| measure_text_width(row[c]))
                .max()
                .unwrap_or(0)
        })
        .collect();

    rows.iter()
        .map(|row| {
            let mut line = String::new();

            for (c, column) in columns.iter().enumerate() {
                let cell = row[c];
                let padding = " ".repeat(widths[c].saturating_sub(measure_text_width(cell)));

                match (column.alignment, column.separator_position) {
                    // Left, separator hugs the content: content, separator, padding.
                    (ColumnAlignment::Left, SeparatorPosition::AfterContent) => {
                        line.push_str(cell);
                        line.push_str(&column.separator);
                        line.push_str(&padding);
                    }
                    // Left, separator before the next column: content, padding, separator.
                    (ColumnAlignment::Left, SeparatorPosition::BeforeNextColumn) => {
                        line.push_str(cell);
                        line.push_str(&padding);
                        line.push_str(&column.separator);
                    }
                    // Right: padding is always on the left, so the separator hugs
                    // the content regardless of its configured position.
                    (ColumnAlignment::Right, _) => {
                        line.push_str(&padding);
                        line.push_str(cell);
                        line.push_str(&column.separator);
                    }
                }
            }

            // Final column: the remainder, rendered verbatim.
            line.push_str(row[columns.len()]);
            line
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn canonical_example_aligns_columns() {
        let items = to_strings(&[
            "copy_current_location: 898.95 KB (2025-10-12 15:41), /path1",
            "summary-gen: 211.29 MB (2025-10-13 20:04), /path2",
            "rona: 1.26 GB (2025-10-14 18:29), /path3",
        ]);

        let columns = vec![
            ColumnConfig::new_with_separator(": ", ColumnAlignment::Left),
            ColumnConfig::new(ColumnAlignment::Left),
            ColumnConfig::new_with_separator(", ", ColumnAlignment::Left),
        ];

        let rows = format_rows(&items, &columns);

        // The widest row is unchanged.
        assert_eq!(
            rows[0],
            "copy_current_location: 898.95 KB (2025-10-12 15:41), /path1"
        );
        assert_eq!(
            rows[1],
            format!(
                "summary-gen:{}211.29 MB (2025-10-13 20:04), /path2",
                " ".repeat(11)
            )
        );
        // "1.26" is split from "GB" on the space, so padding lands between them.
        assert_eq!(
            rows[2],
            format!(
                "rona:{}1.26{}GB (2025-10-14 18:29), /path3",
                " ".repeat(18),
                " ".repeat(3)
            )
        );

        // The timestamp and path columns share an offset across every row.
        for needle in ["(2025-", "/path"] {
            let offset = rows[0].find(needle).unwrap();
            for row in &rows {
                assert_eq!(row.find(needle), Some(offset), "misaligned {needle:?}");
            }
        }
    }

    #[test]
    fn right_aligns_numeric_column() {
        let items = to_strings(&["a 898.95 x", "b 1.26 y"]);

        let columns = vec![
            ColumnConfig::new(ColumnAlignment::Left),
            ColumnConfig::new(ColumnAlignment::Right),
        ];

        let rows = format_rows(&items, &columns);

        assert_eq!(rows[0], "a 898.95 x");
        assert_eq!(rows[1], "b   1.26 y");
    }

    #[test]
    fn custom_separator_hugs_content() {
        let items = to_strings(&["name: value", "longer-name: v"]);

        let columns = vec![ColumnConfig::new_with_separator(
            ": ",
            ColumnAlignment::Left,
        )];

        let rows = format_rows(&items, &columns);

        // "name" is padded up to the width of "longer-name" (11).
        assert_eq!(rows[0], format!("name:{}value", " ".repeat(8)));
        assert_eq!(rows[1], "longer-name: v");
    }

    #[test]
    fn missing_separator_leaves_row_intact() {
        let items = to_strings(&["a: b", "no-separator-here"]);

        let columns = vec![ColumnConfig::new_with_separator(
            ": ",
            ColumnAlignment::Left,
        )];

        let rows = format_rows(&items, &columns);

        // "a" is padded up to the width of "no-separator-here" (17); the ": "
        // separator supplies one of those spaces, the other 16 are padding.
        assert_eq!(rows[0], format!("a:{}b", " ".repeat(17)));
        // Without the separator the whole string is the (final) leftover column.
        assert_eq!(rows[1], "no-separator-here: ");
    }

    #[test]
    fn no_columns_returns_items_untouched() {
        let items = to_strings(&["foo", "bar"]);

        let rows = format_rows(&items, &[]);

        assert_eq!(rows, items);
    }

    #[test]
    fn separator_builder_replaces_separator() {
        let config = ColumnConfig::new(ColumnAlignment::Left).separator(" | ");
        let items = to_strings(&["a | b", "aa | b"]);

        let rows = format_rows(&items, &[config]);

        // The separator hugs the content, padding trails it.
        assert_eq!(rows[0], "a |  b");
        assert_eq!(rows[1], "aa | b");
    }

    #[test]
    fn delimiter_decouples_input_split_from_output_separator() {
        // Input is space-separated; render columns with a wider 3-space gap so
        // the columns are clearly distinguishable.
        let items = to_strings(&[
            "col_1 col_2 long_col_3",
            "long_col_1 col_2 long_col_3",
            "even_longer_col_1 long_col_2 long_col_3",
        ]);

        let columns = vec![
            ColumnConfig::new_with_separator("   ", ColumnAlignment::Left).delimiter(" "),
            ColumnConfig::new_with_separator("   ", ColumnAlignment::Left).delimiter(" "),
        ];

        let rows = format_rows(&items, &columns);

        assert_eq!(
            rows[0],
            format!("col_1{}col_2{}long_col_3", " ".repeat(15), " ".repeat(8))
        );
        assert_eq!(
            rows[1],
            format!(
                "long_col_1{}col_2{}long_col_3",
                " ".repeat(10),
                " ".repeat(8)
            )
        );
        assert_eq!(
            rows[2],
            format!(
                "even_longer_col_1{}long_col_2{}long_col_3",
                " ".repeat(3),
                " ".repeat(3)
            )
        );

        // The second column starts at the same offset (20) in every row.
        assert!(rows[0][20..].starts_with("col_2"));
        assert!(rows[1][20..].starts_with("col_2"));
        assert!(rows[2][20..].starts_with("long_col_2"));
    }

    #[test]
    fn separator_before_next_column_aligns_dividers() {
        let items = to_strings(&[
            "col_1 col_2 long_col_3",
            "long_col_1 col_2 long_col_3",
            "even_longer_col_1 long_col_2 long_col_3",
        ]);

        let columns = vec![
            ColumnConfig::new_with_separator(" | ", ColumnAlignment::Left)
                .delimiter(" ")
                .separator_position(SeparatorPosition::BeforeNextColumn),
            ColumnConfig::new_with_separator(" - ", ColumnAlignment::Left)
                .delimiter(" ")
                .separator_position(SeparatorPosition::BeforeNextColumn),
        ];

        let rows = format_rows(&items, &columns);

        assert_eq!(
            rows[0],
            format!(
                "col_1{} | col_2{} - long_col_3",
                " ".repeat(12),
                " ".repeat(5)
            )
        );
        assert_eq!(
            rows[1],
            format!(
                "long_col_1{} | col_2{} - long_col_3",
                " ".repeat(7),
                " ".repeat(5)
            )
        );
        assert_eq!(rows[2], "even_longer_col_1 | long_col_2 - long_col_3");

        // Both dividers form their own aligned column.
        for row in &rows {
            assert_eq!(row.find(" | "), rows[0].find(" | "));
            assert_eq!(row.find(" - "), rows[0].find(" - "));
        }
    }

    #[test]
    fn semicolon_delimited_with_right_aligned_divider_column() {
        // Mirrors the third case in `examples/tabular.rs`.
        let items = to_strings(&["Alice;30;Engineer", "Bob;7;Student", "Charlie;42;Astronaut"]);

        let columns = vec![
            ColumnConfig::new(ColumnAlignment::Left)
                .separator(" : ")
                .delimiter(";")
                .separator_position(SeparatorPosition::BeforeNextColumn),
            ColumnConfig::new(ColumnAlignment::Right)
                .separator(" : ")
                .delimiter(";")
                .separator_position(SeparatorPosition::BeforeNextColumn),
        ];

        let rows = format_rows(&items, &columns);

        assert_eq!(rows[0], "Alice   : 30 : Engineer");
        assert_eq!(rows[1], "Bob     :  7 : Student");
        assert_eq!(rows[2], "Charlie : 42 : Astronaut");
    }

    #[test]
    fn delimiter_defaults_to_separator() {
        let items = to_strings(&["a: b", "aa: b"]);

        // No explicit delimiter: the ": " separator also splits the input.
        let with_default = format_rows(
            &items,
            &[ColumnConfig::new_with_separator(
                ": ",
                ColumnAlignment::Left,
            )],
        );
        let with_explicit = format_rows(
            &items,
            &[ColumnConfig::new_with_separator(": ", ColumnAlignment::Left).delimiter(": ")],
        );

        assert_eq!(with_default, with_explicit);
    }

    #[test]
    fn new_matches_new_with_space_separator() {
        let items = to_strings(&["a b", "aa b"]);

        let default = format_rows(&items, &[ColumnConfig::new(ColumnAlignment::Left)]);
        let explicit = format_rows(
            &items,
            &[ColumnConfig::new_with_separator(" ", ColumnAlignment::Left)],
        );

        assert_eq!(default, explicit);
    }
}
