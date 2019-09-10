#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::all)]

extern crate json;
extern crate nom;

#[cfg(feature = "std")]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "std")]
extern crate regex;

#[cfg(all(not(feature = "std")))]
#[macro_use]
extern crate alloc;

/// A facade around all the types we need from the `std`, `core`, and `alloc`
/// crates. This avoids elaborate import wrangling having to happen in every
/// module.
pub mod lib {
	mod core {
		#[cfg(not(feature = "std"))]
		pub use core::*;
		#[cfg(feature = "std")]
		pub use std::*;
	}

	#[cfg(not(feature = "std"))]
	/// internal std exports for no_std compatibility
	pub mod std {
		pub use alloc::{boxed, string, vec};
		pub use core::{borrow, cmp, convert, fmt, iter, mem, ops, option, result, slice, str};
		/// internal reproduction of std prelude
		pub mod prelude {
			pub use core::prelude as v1;
		}
	}

	#[cfg(feature = "std")]
	/// internal std exports for no_std compatibility
	pub mod std {
		pub use std::{
			alloc, borrow, boxed, cmp, collections, convert, fmt, hash, iter, mem, ops, option,
			result, slice, str, string, vec,
		};
		/// internal reproduction of std prelude
		pub mod prelude {
			pub use std::prelude as v1;
		}
	}
}

mod did_parser;
mod diddoc_parser;

pub mod did;
pub mod did_doc;
pub mod macros;

pub use json::parse as json_parse;
