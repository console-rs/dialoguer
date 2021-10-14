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
//! * Selections prompts (single and multi)
//! * Fuzzy select prompt
//! * Other kind of prompts
//! * Editor launching

pub use console;
#[cfg(feature = "editor")]
pub use edit::Editor;
#[cfg(feature = "history")]
pub use history::History;
use paging::Paging;
pub use prompts::{
    confirm::Confirm, input::Input, multi_select::MultiSelect, select::Select, sort::Sort,
};
pub use validate::Validator;

#[cfg(feature = "fuzzy-select")]
pub use prompts::fuzzy_select::FuzzySelect;

#[cfg(feature = "password")]
pub use prompts::password::Password;

#[cfg(feature = "editor")]
mod edit;
#[cfg(feature = "history")]
mod history;
mod paging;
mod prompts;
pub mod theme;
mod validate;
