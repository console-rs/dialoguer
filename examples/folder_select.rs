use dialoguer::{theme::ColorfulTheme, FolderSelect};

fn main() {
    let selection = FolderSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select some sobfolder from /tmp")
        .folder("/tmp")
        .interact()
        .unwrap();

    println!("Folder you selected: {}", selection);

    let selection = FolderSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select some file from /tmp")
        .folder("/tmp")
        .file(true)
        .interact()
        .unwrap();

    println!("File you selected: {}", selection);
}
