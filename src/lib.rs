// #![no_main]
// #![no_std]
#![warn(clippy::all)]

extern crate json;
#[macro_use]
extern crate lazy_static;
extern crate nom;
extern crate regex;

mod did_parser;
mod diddoc_parser;

pub mod did;
pub mod did_doc;
pub mod macros;

pub use json::parse as json_parse;
