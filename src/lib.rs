//! dialoguer is a library for Rust that helps you build useful small
//! interactive user inputs for the command line.  It provides utilities
//! to render various simple dialogs like confirmation prompts, text
//! inputs and more.
//!
//! Best paired with other libraries in the family:
//!
//! * [indicatif](https://docs.rs/indicatif)
//! * [console](https://docs.rs/console)
//!
//! # Crate Contents
//!
//! * Confirmation prompts
//! * Input prompts (regular and password)
//! * Input validation
//! * Menu selections
//! * Checkboxes
//! * Editor launching
extern crate console;
#[cfg(feature = "validation")]
extern crate regex;
extern crate tempfile;
pub use edit::Editor;
pub use prompts::{Confirmation, Input, PasswordInput, ValidatedInput};
pub use select::{Checkboxes, Select};

mod edit;
mod prompts;
mod select;
pub mod theme;
pub mod validate;
