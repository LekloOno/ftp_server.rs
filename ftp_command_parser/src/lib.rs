//! Defines an environment for run-time flexible, extensible, and specializable FTP command parser.

pub mod parser;
pub mod command;
pub mod error;
pub mod dummy_impl;

pub use error::{Error, Result};