// #![no_main]
// #![no_std]
#![warn(clippy::all)]

extern crate json;
extern crate nom;

mod did_parser;
mod diddoc_parser;

pub mod did;
pub mod did_doc;
pub mod macros;

pub use json::parse as json_parse;
