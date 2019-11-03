extern crate dialoguer;

use dialoguer::{DateTimeSelect};

fn main() {
    let datetime = DateTimeSelect::new()
        .with_prompt("Pick a time")
        .default("2019-01-01T00:00:00-08:00")
        .weekday(true)
        .interact()
        .unwrap();
    println!("Time selected {}", datetime);
}

