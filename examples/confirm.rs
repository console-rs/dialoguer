extern crate dialoguer;

use dialoguer::Confirm;

fn main() {
    println!("with confirm");
    if Confirm::new()
        .with_prompt("continue?")
        .confirm(true)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("continuing");
    } else {
        println!("exiting");
    }

    println!();

    println!("without confirm");
    if Confirm::new()
        .with_prompt("continue?")
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("continuing");
    } else {
        println!("exiting");
    }
}
