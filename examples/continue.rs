extern crate dialoguer;

use dialoguer::{Confirm, Input};

fn main() {
    if Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
        return;
    }

    let input: String = Input::new().with_prompt("Your name").interact().unwrap();
    println!("Hello {}!", input);
}
