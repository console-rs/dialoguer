extern crate dialoguer;

use dialoguer::Confirmation;

fn main() {
    if Confirmation::new("Do you want to continue?").interact().unwrap() {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }
}
