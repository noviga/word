pub(crate) use nouns::*;

mod nouns;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accented() {
        assert_eq!(ABAZHUR.accented().collect::<String>(), "АБАЖУ́Р");
    }
}
