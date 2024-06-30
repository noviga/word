use std::str::Chars;

/// A word in the language.
pub struct Word {
    /// The word itself.
    pub word: &'static str,
    /// The accent position.
    pub accent: Option<usize>,
    /// The part of speech.
    pub part_of_speech: PartOfSpeech,
    /// The meanings.
    pub meanings: &'static [&'static str],
}

/// The part of speech.
pub enum PartOfSpeech {
    /// An adjective.
    Adjective,
    /// An adverb.
    Adverb,
    /// A conjunction.
    Conjunction,
    /// An interjection.
    Interjection,
    /// A noun.
    Noun,
    /// A preposition.
    Preposition,
    /// A pronoun.
    Pronoun,
    /// A verb.
    Verb,
}

pub struct AccentedWord<'a> {
    chars: Chars<'a>,
    accent: Option<usize>,
    current: usize,
}

impl Word {
    /// Creates a new adjective.
    pub const fn adj(word: &'static str, meanings: &'static [&'static str]) -> Self {
        Self {
            word,
            accent: None,
            part_of_speech: PartOfSpeech::Adjective,
            meanings,
        }
    }

    /// Creates a new adverb.
    pub const fn adv(word: &'static str, meanings: &'static [&'static str]) -> Self {
        Self {
            word,
            accent: None,
            part_of_speech: PartOfSpeech::Adverb,
            meanings,
        }
    }

    /// Creates a new conjunction.
    pub const fn conj(word: &'static str, meanings: &'static [&'static str]) -> Self {
        Self {
            word,
            accent: None,
            part_of_speech: PartOfSpeech::Conjunction,
            meanings,
        }
    }

    /// Creates a new interjection.
    pub const fn interj(word: &'static str, meanings: &'static [&'static str]) -> Self {
        Self {
            word,
            accent: None,
            part_of_speech: PartOfSpeech::Interjection,
            meanings,
        }
    }

    /// Creates a new noun.
    pub const fn noun(word: &'static str, meanings: &'static [&'static str]) -> Self {
        Self {
            word,
            accent: None,
            part_of_speech: PartOfSpeech::Noun,
            meanings,
        }
    }

    /// Creates a new preposition.
    pub const fn prep(word: &'static str, meanings: &'static [&'static str]) -> Self {
        Self {
            word,
            accent: None,
            part_of_speech: PartOfSpeech::Preposition,
            meanings,
        }
    }

    /// Creates a new pronoun.
    pub const fn pronoun(word: &'static str, meanings: &'static [&'static str]) -> Self {
        Self {
            word,
            accent: None,
            part_of_speech: PartOfSpeech::Preposition,
            meanings,
        }
    }

    /// Creates a new verb.
    pub const fn verb(word: &'static str, meanings: &'static [&'static str]) -> Self {
        Self {
            word,
            accent: None,
            part_of_speech: PartOfSpeech::Preposition,
            meanings,
        }
    }

    /// Sets the accent position.
    pub const fn with_accent(mut self, accent: usize) -> Self {
        self.accent = Some(accent);
        self
    }

    /// Returns the word with the accent.
    pub fn accented(&self) -> AccentedWord {
        AccentedWord {
            chars: self.word.chars(),
            accent: self.accent,
            current: 0,
        }
    }
}

impl Iterator for AccentedWord<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(accent) = self.accent {
            let next = if self.current == accent + 1 {
                Some('\u{301}')
            } else {
                self.chars.next()
            };
            self.current += 1;
            next
        } else {
            self.chars.next()
        }
    }
}
