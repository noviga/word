use crate::Word;

#[cfg(feature = "russian")]
use crate::russian::*;

/// All words.
pub const ALL: &[&Word] = &[
    #[cfg(feature = "russian")]
    &ABAZHUR,
    #[cfg(feature = "russian")]
    &BAL,
];
