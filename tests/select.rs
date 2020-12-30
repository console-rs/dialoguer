use dialoguer::{theme::ColorfulTheme, Select};

use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;

#[test]
fn basic_navigation_produces_correct_selection() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('j'));
    enigo.key_down(Key::Return);
    thread::sleep(Duration::from_millis(10));
    enigo.key_up(Key::Return);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    assert_eq!(Some(1), selection);
}
