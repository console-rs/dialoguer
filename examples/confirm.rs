extern crate dialoguer;

use dialoguer::Confirm;

fn main() {
    println!("disable default");
    if Confirm::new()
        .with_prompt("continue?")
        .disable_default(true)
        .interact()
        .unwrap()
    {
        println!("continuing");
    } else {
        println!("exiting");
    }

    println!();

    println!("enable default");
    if Confirm::new()
        .with_prompt("continue?")
        .interact()
        .unwrap()
    {
        println!("continuing");
    } else {
        println!("exiting");
    }
}
