extern crate console;
extern crate dialoguer;

use std::error::Error;
use std::net::IpAddr;

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirmation, Input, Select};

#[derive(Debug)]
struct Config {
    interface: IpAddr,
    hostname: String,
    use_acme: bool,
    private_key: Option<String>,
    cert: Option<String>,
}

fn init_config() -> Result<Option<Config>, Box<Error>> {
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
        .interact()?
    {
        return Ok(None);
    }

    let interface = Input::with_theme(&theme)
        .with_prompt("Interface")
        .default("127.0.0.1".parse().unwrap())
        .interact()?;

    let hostname = Input::with_theme(&theme)
        .with_prompt("Hostname")
        .interact()?;

    let tls = Select::with_theme(&theme)
        .with_prompt("Configure TLS")
        .default(0)
        .item("automatic with ACME")
        .item("manual")
        .item("no")
        .interact()?;

    let (private_key, cert, use_acme) = match tls {
        0 => (Some("acme.pkey".into()), Some("acme.cert".into()), true),
        1 => (
            Some(
                Input::with_theme(&theme)
                    .with_prompt("  Path to private key")
                    .interact()?,
            ),
            Some(
                Input::with_theme(&theme)
                    .with_prompt("  Path to certificate")
                    .interact()?,
            ),
            false,
        ),
        _ => (None, None, false),
    };

    Ok(Some(Config {
        hostname,
        interface,
        private_key,
        cert,
        use_acme,
    }))
}

fn main() {
    match init_config() {
        Ok(None) => println!("Aborted."),
        Ok(Some(config)) => println!("{:#?}", config),
        Err(err) => println!("error: {}", err),
    }
}
