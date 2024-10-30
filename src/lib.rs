mod macros;
pub mod parse;
pub mod string;

#[cfg(test)]
mod tests {
    use crate::parse::*;
    use crate::string::*;

    #[test]
    fn test_to_string() {
        assert_eq!(to_string(NOUN), "N", "failed");
        assert_eq!(to_string(CONDITIONAL | CRASIS), "COND-K", "failed");
        assert_eq!(to_string(ARTICLE | NOMINATIVE), "T-N", "failed");
        assert_eq!(to_string(ARTICLE | GENITIVE | SINGULAR), "T-GS", "failed");
        assert_eq!(
            to_string(ARTICLE | GENITIVE | SINGULAR | NEUTER),
            "T-GSN",
            "failed"
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(to_string(NOUN), "N", "failed");
        assert_eq!(to_string(CONDITIONAL | CRASIS), "COND-K", "failed");
        assert_eq!(to_string(ARTICLE | NOMINATIVE), "T-N", "failed");
        assert_eq!(to_string(ARTICLE | GENITIVE | SINGULAR), "T-GS", "failed");
        assert_eq!(
            to_string(ARTICLE | GENITIVE | SINGULAR | NEUTER),
            "T-GSN",
            "failed"
        );
    }
}
