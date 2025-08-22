# dialoguer

[![Build Status](https://github.com/console-rs/dialoguer/workflows/CI/badge.svg)](https://github.com/console-rs/dialoguer/actions?query=branch%3Amaster)
[![Latest version](https://img.shields.io/crates/v/dialoguer.svg)](https://crates.io/crates/dialoguer)
[![Documentation](https://docs.rs/dialoguer/badge.svg)](https://docs.rs/dialoguer)

A rust library for command line prompts and similar things.

Best paired with other libraries in the family:

* [console](https://github.com/console-rs/console)
* [indicatif](https://github.com/console-rs/indicatif)

## Usage

Add the library to your `Cargo.toml`:

```shell
cargo add dialoguer
```

## Examples

### Confirm

Docs: [dialoguer::Confirm](https://docs.rs/dialoguer/latest/dialoguer/struct.Confirm.html)

```rust
use dialoguer::{theme::ColorfulTheme, Confirm};

if Confirm::with_theme(&ColorfulTheme::default())
    .with_prompt("Do you want to continue?")
    .interact()?
{
    println!("Looks like you want to continue");
}
```

![confirm](https://vhs.charm.sh/vhs-5ianSRV6gBIQw8zHbXZs7X.gif)

With a default value:

```rust
use dialoguer::{theme::ColorfulTheme, Confirm};

if Confirm::new()
    .with_prompt("Do you want to continue?")
    .default(true)
    .interact()?
{
    println!("Looks like you want to continue");
}
```

![confirm-with-default](https://vhs.charm.sh/vhs-KumYDsqM2KSxaMUHRr8IV.gif)

## Input

Docs: [dialoguer::Input](https://docs.rs/dialoguer/latest/dialoguer/struct.Input.html)

```rust
use dialoguer::{theme::ColorfulTheme, Input};

let name: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
    .with_prompt("What is your name?")
    .interact()?;
println!("Hello, {name}");
```

![input](https://vhs.charm.sh/vhs-7EYUy5VCybcotdxrL8QCXk.gif)

## Password

Docs: [dialoguer::Password](https://docs.rs/dialoguer/latest/dialoguer/struct.Password.html)

```rust
use dialoguer::{theme::ColorfulTheme, Password};

let password: String = Password::with_theme(&ColorfulTheme::default())
    .with_prompt("Enter your password")
    .interact()?;
println!("Your password is: {password}");
```

![password](https://vhs.charm.sh/vhs-1HTgKYmFc09dNtuHu5hWOO.gif)

## Editor

Docs: [dialoguer::Editor](https://docs.rs/dialoguer/latest/dialoguer/struct.Editor.html)

```rust
use dialoguer::Editor;

match dialoguer::Editor::new().edit("Some content")? {
    Some(content) => println!("Content: {content:?}"),
    None => println!("File was not saved"),
}
```

![editor](https://vhs.charm.sh/vhs-3DISbkWUNwMms076djOQ3e.gif)

## Select

Docs: [dialoguer::Select](https://docs.rs/dialoguer/latest/dialoguer/struct.Select.html)

```rust
use dialoguer::{theme::ColorfulTheme, Select};

let items = vec!["Apple", "Banana", "Cherry"];
let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("What is your favourite fruit?")
    .items(&items)
    .interact()?;
println!("You picked: {selection}", selection = items[selection]);
```

![select](https://vhs.charm.sh/vhs-3ylAvmWOIiBkYexnG7j4F9.gif)

## FuzzySelect

Docs: [dialoguer::FuzzySelect](https://docs.rs/dialoguer/latest/dialoguer/struct.FuzzySelect.html)

```rust
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

let items = vec!["Apple", "Banana", "Cherry"];
let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
    .with_prompt("What is your favourite fruit?")
    .items(&items)
    .interact()?;
println!("You picked: {selection}", selection = items[selection]);
```

![fuzzy-select](https://vhs.charm.sh/vhs-3JUdbUNwnUKWVjk6J5XoKh.gif)

## MultiSelect

Docs: [dialoguer::MultiSelect](https://docs.rs/dialoguer/latest/dialoguer/struct.MultiSelect.html)

```rust
use dialoguer::{theme::ColorfulTheme, MultiSelect};

let items = vec!["Apple", "Banana", "Cherry"];
let selection = MultiSelect::with_theme(&ColorfulTheme::default())
    .with_prompt("What are your favourite fruits?")
    .items(&items)
    .interact()?;
let selected_items: Vec<_> = selection.iter().map(|i| items[*i]).collect();
println!("You picked: {selected_items:?}");
```

![multi-select](https://vhs.charm.sh/vhs-5Jje1Pdxsw4w5jLJjeWNbI.gif)

## Sort

Docs: [dialoguer::Sort](https://docs.rs/dialoguer/latest/dialoguer/struct.Sort.html)

```rust
use dialoguer::{theme::ColorfulTheme, Sort};

let items = vec!["Apple", "Banana", "Cherry"];
let selection = Sort::with_theme(&ColorfulTheme::default())
    .with_prompt("Sort the fruits")
    .items(&items)
    .interact()?;
let sorted_items: Vec<_> = selection.iter().map(|i| items[*i]).collect();
println!("You sorted: {sorted_items:?}");
```

![sort](https://vhs.charm.sh/vhs-mcxq0aABXECgIdafLBNZN.gif)

## License and Links

* [Documentation](https://docs.rs/dialoguer/)
* [Issue Tracker](https://github.com/console-rs/dialoguer/issues)
* [Examples](https://github.com/console-rs/dialoguer/tree/master/examples)
* License: [MIT](https://github.com/console-rs/dialoguer/blob/main/LICENSE)
