use dialoguer::Confirm;

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
    if Confirm::new().with_prompt("continue?").interact().unwrap() {
        println!("continuing");
    } else {
        println!("exiting");
    }
}
