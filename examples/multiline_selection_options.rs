extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Select, FuzzySelect, Checkboxes};

const selections: [&str; 100] = [
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
    "Option A\n  This is an option",
    "Option B\n  This is another option",
    "Option C\n  A better option",
    "Option D\n  Something else",
    "Option E\n  Last but not least",
];

fn main() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your favourite option")
        .default(0)
        .offset(1)
        .paged(true)
        .lines_per_item(2)
        .items(&selections)
        .interact()
        .unwrap();
    println!("Enjoy your {}!", selections[selection]);

    let fuzzy_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("FuzzySelect your favourite option")
        .default(0)
        .offset(1)
        .paged(true)
        .show_match(false)
        .lines_per_item(2)
        .ignore_casing(false)
        .items(&selections)
        .interact()
        .unwrap();
    println!("Enjoy your {}!", fuzzy_selection);

    let check_selects = Checkboxes::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your favourite option")
        .offset(1)
        .paged(true)
        .lines_per_item(2)
        .items(&selections)
        .interact()
        .unwrap();
    
    if check_selects.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in check_selects {
            println!("  {}", selections[selection]);
        }
    }
}

