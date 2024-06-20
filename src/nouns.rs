use crate::Word;

#[cfg(feature = "russian")]
use crate::russian::*;

/// Nouns.
pub const NOUNS: &[&Word] = &[
    #[cfg(feature = "russian")]
    &ABAZHUR,
    #[cfg(feature = "russian")]
    &BAL,
];
