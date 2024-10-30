// Part of speech bits 1-5
pub const UNKNOWN: u32 = 0;
pub const PARTICLE: u32 = 1;
pub const VERB: u32 = 2;
pub const NOUN: u32 = 3;
pub const ADJECTIVE: u32 = 4;
pub const ADVERB: u32 = 5;
pub const CONJUNCTION: u32 = 6;
pub const PROPER_NOUN: u32 = 7;
pub const PREPOSITION: u32 = 8;
pub const CONDITIONAL: u32 = 9;
pub const ARTICLE: u32 = 10;
pub const INTERJECTION: u32 = 11;
pub const PRONOUN: u32 = 12;
pub const PERSONAL_PRONOUN: u32 = 13;
pub const POSSESSIVE_PRONOUN: u32 = 14;
pub const RELATIVE_PRONOUN: u32 = 15;
pub const DEMONSTRATIVE_PRONOUN: u32 = 16;
pub const RECIPROCAL_PRONOUN: u32 = 17;
pub const REFLEXIVE_PRONOUN: u32 = 18;
pub const TRANSLITERATION: u32 = 19;
pub const HEBREW_TRANSLITERATION: u32 = 20;
pub const ARAMAIC_TRANSLITERATION: u32 = 21;
pub const LETTER: u32 = 22;
pub const NUMERAL: u32 = 23;
pub const SUPERLATIVE_ADJECTIVE: u32 = 24;
pub const SUPERLATIVE_ADVERB: u32 = 25;
pub const SUPERLATIVE_NOUN: u32 = 26;
pub const COMPARATIVE_ADJECTIVE: u32 = 27;
pub const COMPARATIVE_ADVERB: u32 = 28;
pub const COMPARATIVE_NOUN: u32 = 29;

pub fn part_of_speech(p: u32) -> u32 {
    p & 0b11111
}

pub fn set_part_of_speech(p: u32, pos: u32) -> u32 {
    let mask = !0b11111;
    (p & mask) | pos
}

// Flags that modify part of speech, bits 6-11
pub const INTERROGATIVE: u32 = 1 << 6;
pub const NEGATIVE: u32 = 1 << 7;
pub const CORRELATIVE: u32 = 1 << 8;
pub const INDEFINITE: u32 = 1 << 9;
pub const INDECLINABLE: u32 = 1 << 10;
pub const CRASIS: u32 = 1 << 11;

pub fn is_interrogative(p: u32) -> bool {
    p & INTERROGATIVE == INTERROGATIVE
}

pub fn is_negative(p: u32) -> bool {
    p & NEGATIVE == NEGATIVE
}

pub fn is_correlative(p: u32) -> bool {
    p & CORRELATIVE == CORRELATIVE
}

pub fn is_indefinite(p: u32) -> bool {
    p & INDEFINITE == INDEFINITE
}

pub fn is_indeclinable(p: u32) -> bool {
    p & INDECLINABLE == INDECLINABLE
}

pub fn is_crasis(p: u32) -> bool {
    p & CRASIS == CRASIS
}

// Tense form, 4 bits. Bits 12-15
pub const PRESENT: u32 = 1 << 12;
pub const FUTURE: u32 = 2 << 12;
pub const AORIST: u32 = 3 << 12;
pub const IMPERFECT: u32 = 4 << 12;
pub const PERFECT: u32 = 5 << 12;
pub const PLUPERFECT: u32 = 6 << 12;
pub const SECOND_FUTURE: u32 = 7 << 12;
pub const SECOND_AORIST: u32 = 8 << 12;
pub const SECOND_PERFECT: u32 = 9 << 12;
pub const SECOND_PLUPERFECT: u32 = 10 << 12;

pub fn tense(p: u32) -> u32 {
    p & (0b1111 << 12)
}

// Tense form bits are overloaded to hold
// data for non verbs
pub const REF_SINGULAR: u32 = 0 << 12;
pub const REF_PLURAL: u32 = 1 << 12;

pub fn ref_number(p: u32) -> u32 {
    p & (0b1 << 12)
}

// Gender, 3 bits, 16-18
pub const MASCULINE: u32 = 1 << 16;
pub const FEMININE: u32 = 2 << 16;
pub const NEUTER: u32 = 4 << 16;

pub fn gender(p: u32) -> u32 {
    p & (0b111 << 16)
}

// Case, 3 bits, 19-21
pub const NOMINATIVE: u32 = 1 << 19;
pub const ACCUSATIVE: u32 = 2 << 19;
pub const GENITIVE: u32 = 3 << 19;
pub const DATIVE: u32 = 4 << 19;
pub const VOCATIVE: u32 = 5 << 19;

pub fn case(p: u32) -> u32 {
    p & (0b111 << 19)
}

// Voice, 3 bits, 22-24
pub const ACTIVE_VOICE: u32 = 1 << 22;
pub const MIDDLE_VOICE: u32 = 2 << 22;
pub const PASSIVE_VOICE: u32 = 3 << 22;
pub const MIDDLE_PASSIVE_VOICE: u32 = 4 << 22;
pub const MIDDLE_DEPONENT_VOICE: u32 = 5 << 22;
pub const PASSIVE_DEPONENT_VOICE: u32 = 6 << 22;
pub const MIDDLE_PASSIVE_DEPONENT_VOICE: u32 = 7 << 22;

pub fn voice(p: u32) -> u32 {
    p & (0b111 << 22)
}

// Mood, 3 bits, 25-27
pub const INDICATIVE_MOOD: u32 = 1 << 25;
pub const SUBJUNCTIVE_MOOD: u32 = 2 << 25;
pub const OPTATIVE_MOOD: u32 = 3 << 25;
pub const IMPERATIVE_MOOD: u32 = 4 << 25;
pub const INFINITIVE_MOOD: u32 = 5 << 25;
pub const PARTICIPLE_MOOD: u32 = 6 << 25;

pub fn mood(p: u32) -> u32 {
    p & (0b111 << 25)
}

// Person, 2 bits, 28-29
pub const FIRST_PERSON: u32 = 1 << 28;
pub const SECOND_PERSON: u32 = 2 << 28;
pub const THIRD_PERSON: u32 = 3 << 28;

pub fn person(p: u32) -> u32 {
    p & (0b11 << 28)
}

// Number, 2 bits, 30-31
pub const SINGULAR: u32 = 1 << 30;
pub const DUAL: u32 = 2 << 30;
pub const PLURAL: u32 = 3 << 30;

pub fn number(p: u32) -> u32 {
    p & (0b11 << 30)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_bits() {
        assert_eq!(gender(MASCULINE | NOMINATIVE), MASCULINE);
        assert_eq!(gender(FEMININE | NOMINATIVE), FEMININE);
        assert_eq!(gender(NEUTER | NOMINATIVE), NEUTER);
        assert_eq!(gender(MASCULINE | ACCUSATIVE), MASCULINE);
        assert_eq!(gender(FEMININE | ACCUSATIVE), FEMININE);
        assert_eq!(gender(NEUTER | ACCUSATIVE), NEUTER);
        assert_eq!(gender(MASCULINE | GENITIVE), MASCULINE);
        assert_eq!(gender(FEMININE | GENITIVE), FEMININE);
        assert_eq!(gender(NEUTER | GENITIVE), NEUTER);

        assert_eq!(case(MASCULINE | NOMINATIVE), NOMINATIVE);
        assert_eq!(case(FEMININE | NOMINATIVE), NOMINATIVE);
        assert_eq!(case(NEUTER | NOMINATIVE), NOMINATIVE);
        assert_eq!(case(MASCULINE | ACCUSATIVE), ACCUSATIVE);
        assert_eq!(case(FEMININE | ACCUSATIVE), ACCUSATIVE);
        assert_eq!(case(NEUTER | ACCUSATIVE), ACCUSATIVE);
        assert_eq!(case(MASCULINE | GENITIVE), GENITIVE);
        assert_eq!(case(FEMININE | GENITIVE), GENITIVE);
        assert_eq!(case(NEUTER | GENITIVE), GENITIVE);

        let parsed = DEMONSTRATIVE_PRONOUN | ACCUSATIVE | PLURAL | NEUTER;
        assert_eq!(is_crasis(parsed), false);
        assert_eq!(case(parsed), ACCUSATIVE);
        assert_eq!(number(parsed), PLURAL);
        assert_eq!(gender(parsed), NEUTER);
        assert_eq!(part_of_speech(parsed), DEMONSTRATIVE_PRONOUN);
        let parsed = parsed | CRASIS;
        assert_eq!(is_crasis(parsed), true);
        assert_eq!(case(parsed), ACCUSATIVE);
        assert_eq!(number(parsed), PLURAL);
        assert_eq!(gender(parsed), NEUTER);
        assert_eq!(part_of_speech(parsed), DEMONSTRATIVE_PRONOUN);
        assert_eq!(
            parsed,
            DEMONSTRATIVE_PRONOUN | ACCUSATIVE | PLURAL | NEUTER | CRASIS
        );

        let parsed = COMPARATIVE_ADJECTIVE | INTERROGATIVE;
        assert_eq!(is_crasis(parsed), false);
        assert_eq!(is_interrogative(parsed), true);

        let parsed = COMPARATIVE_NOUN | INTERROGATIVE;
        assert_eq!(is_crasis(parsed), false);
        assert_eq!(is_interrogative(parsed), true);
        assert_eq!(part_of_speech(parsed), COMPARATIVE_NOUN);
    }

    #[test]
    fn test_set_part_of_speech() {
        let parsed = NOUN; // == 3
        let out = set_part_of_speech(parsed, VERB); // == 2
        assert_eq!(part_of_speech(out), VERB, "hmmm {} {}", parsed, out);

        let parsed = ADJECTIVE;
        let out = set_part_of_speech(parsed, SUPERLATIVE_ADJECTIVE);
        assert_eq!(
            part_of_speech(out),
            SUPERLATIVE_ADJECTIVE,
            "hmmm {} {}",
            parsed,
            out
        );
    }
}
