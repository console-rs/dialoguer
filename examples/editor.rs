#[cfg(feature = "editor")]
fn main() {
    use dialoguer::Editor;
    if let Some(rv) = Editor::new().edit("Enter a commit message").unwrap() {
        println!("Your message:");
        println!("{}", rv);
    } else {
        println!("Abort!");
    }
}

#[cfg(not(feature = "editor"))]
fn main() {
    println!("editor feature must be enabled.");
}
