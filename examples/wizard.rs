extern crate console;
extern crate dialoguer;

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirmation, Input, Select};

fn main() {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        indicator_style: Style::new().yellow().bold(),
        yes_style: Style::new().yellow().dim(),
        no_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to the setup wizard");

    if !Confirmation::with_theme(&theme)
        .with_text("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("aborting.");
        return;
    }

    let interface = Input::with_theme(&theme)
        .with_prompt("Interface")
        .default("127.0.0.1")
        .interact()
        .unwrap();

    let tls = Select::with_theme(&theme)
        .with_prompt("Configure TLS")
        .default(0)
        .item("automatic with ACME")
        .item("manual")
        .item("no")
        .interact()
        .unwrap();

    let (private_key, cert) = match tls {
        0 => (Some("acme.pkey".into()), Some("acme.cert".into())),
        1 => (
            Some(
                Input::with_theme(&theme)
                    .with_prompt("  Path to private key")
                    .interact()
                    .unwrap(),
            ),
            Some(
                Input::with_theme(&theme)
                    .with_prompt("  Path to certificate")
                    .interact()
                    .unwrap(),
            ),
        ),
        _ => (None, None),
    };

    println!(
        "Binding to {} (pkey = {}, cert = {})",
        interface,
        private_key.as_ref().map(|x| x.as_str()).unwrap_or("-"),
        cert.as_ref().map(|x| x.as_str()).unwrap_or("-")
    );
}
