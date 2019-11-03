extern crate dialoguer;

use dialoguer::{DateTimeSelect};

fn main() {
    let datetime = DateTimeSelect::new()
        .with_prompt("Pick a datetime")
        .default("2019-01-01T00:00:00-08:00")
        .interact()
        .unwrap();
    println!("Datetime selected {}", datetime);

    let date = DateTimeSelect::new()
        .with_prompt("Pick a date")
        .date_type("date")
        .interact()
        .unwrap();
    println!("Date selected {}", date);

    let time = DateTimeSelect::new()
        .with_prompt("Pick a time")
        .date_type("time")
        .weekday(false)
        .interact()
        .unwrap();
    println!("Datetime selected {}", time);
}

