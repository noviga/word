//! # Wordlist
//!
//! A collection of words for use in generating passphrases.
//!
//! ## Usage
//!
//! ```
//! use word::ALL;
//!
//! for word in ALL {
//!    println!("{}", word.word);
//! }
//! ```
//!

pub use crate::{all::ALL, nouns::*, word::Word};

mod all;
mod nouns;
mod word;

#[cfg(feature = "russian")]
pub mod russian;
