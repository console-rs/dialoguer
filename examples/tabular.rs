//! Showcases every `ColumnConfig` knob for aligning selection prompts into
//! columns: alignment, custom separators, a delimiter distinct from the
//! separator, and where the separator sits relative to the padding.
//!
//! Run with: `cargo run --example tabular`

use dialoguer::{
    tabular::{ColumnAlignment, ColumnConfig, SeparatorPosition},
    theme::ColorfulTheme,
    MultiSelect, Select,
};

fn main() {
    // 1. Label-style columns: the ": " separator hugs the name (the default
    //    `SeparatorPosition::AfterContent`), and the numeric column is right
    //    aligned so the units line up.
    //
    //    copy_current_location: 898.95 KB (2025-10-12 15:41), /path1
    //    summary-gen:           211.29 MB (2025-10-13 20:04), /path2
    //    rona:                    1.26 GB (2025-10-14 18:29), /path3
    let projects = &[
        "copy_current_location: 898.95 KB (2025-10-12 15:41), /path1",
        "summary-gen: 211.29 MB (2025-10-13 20:04), /path2",
        "rona: 1.26 GB (2025-10-14 18:29), /path3",
    ];

    let project_columns = vec![
        // Name, delimited and rendered with ": ".
        ColumnConfig::new_with_separator(": ", ColumnAlignment::Left),
        // Size, right-aligned so "1.26 GB" lines up under "898.95 KB".
        ColumnConfig::new(ColumnAlignment::Right),
        // Timestamp, delimited and rendered with ", ".
        ColumnConfig::new_with_separator(", ", ColumnAlignment::Left),
        // The remainder (the path) is the final column.
    ];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select projects to clean (label columns, right-aligned sizes)")
        .items(&projects[..])
        .with_tabular_columns(project_columns)
        .interact()
        .unwrap();

    report("projects", projects, &selections);

    // 2. Divider-style columns: the input is space-separated, but we render a
    //    wider " | " / " - " divider placed *before the next column*
    //    (`SeparatorPosition::BeforeNextColumn`) so the dividers line up into
    //    their own columns.
    //
    //    col_1             | col_2      - long_col_3
    //    long_col_1        | col_2      - long_col_3
    //    even_longer_col_1 | long_col_2 - long_col_3
    let rows = &[
        "col_1 col_2 long_col_3",
        "long_col_1 col_2 long_col_3",
        "even_longer_col_1 long_col_2 long_col_3",
    ];

    let divider_columns = vec![
        ColumnConfig::new_with_separator(" | ", ColumnAlignment::Left)
            .delimiter(" ") // split on a single space, render the wider " | "
            .separator_position(SeparatorPosition::BeforeNextColumn),
        ColumnConfig::new_with_separator(" - ", ColumnAlignment::Left)
            .delimiter(" ")
            .separator_position(SeparatorPosition::BeforeNextColumn),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a row (aligned dividers)")
        .items(&rows[..])
        .with_tabular_columns(divider_columns)
        .interact()
        .unwrap();

    println!("You picked: {}", rows[selection]);

    // 3. The `.separator()` and `.alignment()` builder setters, plus a delimiter
    //    distinct from the separator: split on ";" but render a padded " : "
    //    divider so the colons form their own column. The age column is right
    //    aligned.
    //
    //    Alice   : 30 : Engineer
    //    Bob     :  7 : Student
    //    Charlie : 42 : Astronaut
    let people = &["Alice;30;Engineer", "Bob;7;Student", "Charlie;42;Astronaut"];

    let people_columns = vec![
        ColumnConfig::new(ColumnAlignment::Left)
            .separator(" : ")
            .delimiter(";")
            .separator_position(SeparatorPosition::BeforeNextColumn),
        ColumnConfig::new(ColumnAlignment::Left)
            .alignment(ColumnAlignment::Right)
            .separator(" : ")
            .delimiter(";")
            .separator_position(SeparatorPosition::BeforeNextColumn),
    ];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select people (';'-delimited input, ' : ' dividers)")
        .items(&people[..])
        .with_tabular_columns(people_columns)
        .interact()
        .unwrap();

    report("people", people, &selections);
}

fn report(label: &str, items: &[&str], selections: &[usize]) {
    if selections.is_empty() {
        println!("No {label} selected.");
    } else {
        println!("Selected {label}:");
        for &i in selections {
            println!("  {}", items[i]);
        }
    }
}
