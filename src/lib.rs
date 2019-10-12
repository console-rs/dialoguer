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
//! * Menu fuzzy match selections
extern crate console;
extern crate tempfile;
extern crate regex;
pub use edit::Editor;
pub use prompts::{Confirmation, Input, PasswordInput};
pub use select::{Checkboxes, FuzzySelect, Select};
pub use validate::Validator;

mod edit;
mod prompts;
mod select;
pub mod theme;
mod validate;
