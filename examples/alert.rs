use dialoguer::{theme::ColorfulTheme, Alert};

fn main() {
    let _ = Alert::with_theme(&ColorfulTheme::default())
        .with_prompt("Something went wrong!  Press enter to continue.")
        .interact();

    let _ = Alert::with_theme(&ColorfulTheme::default())
        .with_alert_text("This is an alert, press enter to continue.")
        .interact();

    let _ = Alert::with_theme(&ColorfulTheme::default())
        .with_alert_text("Strange things happened: <spooky error message>.")
        .with_prompt("Press enter to continue.")
        .interact();
}
