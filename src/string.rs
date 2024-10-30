use crate::parse::*;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Incomplete(u32),
    UnexpectedCharacter(u32, char, usize),
    UnknownCase(u32, char),
    UnknownTenseForm(u32, char),
    UnknownPerson(u32, char),
    UnknownNumber(u32, char),
    UnknownVoice(u32, char),
    UnknownGender(u32, char),
    UnknownMood(u32, char),
    UnknownPartOfSpeech(String),
}

pub fn from_string(code: &str) -> Result<u32, ParseError> {
    let data: Vec<char> = code.chars().collect();
    let mut start: usize = 0;
    let mut end: usize = data.len();
    if end == 0 {
        return Ok(UNKNOWN);
    }

    // Silently ignore starting or ending brackets
    if data[start] == '{' || data[start] == '[' {
        start += 1;
    }
    if data[end - 1] == '}' || data[end - 1] == ']' {
        end -= 1;
    }

    if end <= start {
        return Ok(UNKNOWN);
    }

    let data = &data[start..end];
    if data.is_empty() {
        return Ok(UNKNOWN);
    }

    match String::from_iter(data).as_str() {
        "ADV" | "adv" => return Ok(ADVERB),
        "ADV-I" | "adv-i" => return Ok(INTERROGATIVE | ADVERB),
        "ADV-K" | "adv-k" => return Ok(CORRELATIVE | ADVERB),
        "ADV-N" | "adv-n" => return Ok(NEGATIVE | ADVERB),
        "ADV-C" | "adv-c" => return Ok(COMPARATIVE_ADVERB),
        "ADV-S" | "adv-s" => return Ok(SUPERLATIVE_ADVERB),
        "CONJ" | "conj" => return Ok(CONJUNCTION),
        "CONJ-K" | "conj-k" => return Ok(CONJUNCTION | CRASIS),
        "CONJ-P" | "conj-p" => {
            // Appears one time in nestle, Acts 2:18.
            return Ok(CONJUNCTION);
        }
        "CONJ-N" => return Ok(NEGATIVE | CONJUNCTION),
        "COND" => return Ok(CONDITIONAL),
        "COND-K" => return Ok(CONDITIONAL | CRASIS),
        "PRT" => return Ok(PARTICLE),
        "PRT-N" => return Ok(NEGATIVE | PARTICLE),
        "PRT-I" => return Ok(INTERROGATIVE | PARTICLE),
        "PREP" => return Ok(PREPOSITION),
        "INJ" => return Ok(INTERJECTION),
        "ARAM" => return Ok(ARAMAIC_TRANSLITERATION),
        "HEB" => return Ok(HEBREW_TRANSLITERATION),
        "N-PRI" => return Ok(INDECLINABLE | PROPER_NOUN),
        "A-NUI" => return Ok(INDECLINABLE | NUMERAL),
        "N-LI" => return Ok(INDECLINABLE | LETTER),
        "N-OI" => return Ok(INDECLINABLE | NOUN),
        _ => {}
    }

    // Capture the furst few characters up to a -
    let mut next = 0;
    if !data.is_empty() {
        next = 1;
        if data.len() > 1 && data[1] != '-' {
            next = 2;
            if data.len() > 2 && data[2] != '-' {
                next = 3;
                if data.len() > 3 && data[3] != '-' {
                    next = 4;
                    return Err(ParseError::UnknownPartOfSpeech(String::from_iter(
                        data[0..next].iter(),
                    )));
                }
            }
        }
    }

    let p = String::from_iter(data[0..next].iter());
    match p.as_str() {
        "V" | "v" => return vp(VERB, &data[next..]),
        "N" | "n" => return cng(NOUN, &data[next..]),
        "A" | "a" => return cng(ADJECTIVE, &data[next..]),
        "R" | "r" => return cng(RELATIVE_PRONOUN, &data[next..]),
        "C" | "c" => return cng(RECIPROCAL_PRONOUN, &data[next..]),
        "D" | "d" => return cng(DEMONSTRATIVE_PRONOUN, &data[next..]),
        "T" | "t" => return cng(ARTICLE, &data[next..]),
        "O" | "o" => return cng(PRONOUN, &data[next..]),
        "K" | "k" => return cng(CORRELATIVE | PRONOUN, &data[next..]),
        "I" | "i" => return cng(INTERROGATIVE | PRONOUN, &data[next..]),
        "X" | "x" => return cng(INDEFINITE | PRONOUN, &data[next..]),
        "Q" | "q" => return cng(CORRELATIVE | INTERROGATIVE | PRONOUN, &data[next..]),
        "F" | "f" => {
            if data[next] == '-' {
                next += 1;
                if next >= data.len() {
                    return Ok(REFLEXIVE_PRONOUN);
                }
            }
            return cng(REFLEXIVE_PRONOUN | fst(data[next]), &data[next + 1..]);
        }
        "S" | "s" => {
            if data[next] == '-' {
                next += 1;
                if next >= data.len() {
                    return Ok(POSSESSIVE_PRONOUN);
                }
            }
            // What is the meaning of the character data[3]?
            // See https://github.com/byztxt/byzantine-majority-text/issues/10
            if next + 2 >= data.len() {
                return Ok(POSSESSIVE_PRONOUN);
            }
            return cng(
                POSSESSIVE_PRONOUN | fst(data[next]) | ref_n(data[next + 1]),
                &data[next + 2..],
            );
        }
        "P" | "p" => return pcn(PERSONAL_PRONOUN, &data[next..]),
        "PN" | "pn" => return cng(PROPER_NOUN, &data[next..]),
        "IPN" | "ipn" => return cng(INDECLINABLE | PROPER_NOUN, &data[next..]),
        _ => {}
    }

    Err(ParseError::UnknownPartOfSpeech(p))
}

pub fn to_string(parsed: u32) -> String {
    let pos = part_of_speech(parsed);

    match pos {
        ADVERB => {
            if is_interrogative(parsed) {
                return "ADV-I".to_string();
            }
            if is_negative(parsed) {
                return "ADV-N".to_string();
            }
            if is_correlative(parsed) {
                return "ADV-K".to_string();
            }
            return "ADV".to_string();
        }
        COMPARATIVE_ADVERB => return "ADV-C".to_string(),
        SUPERLATIVE_ADVERB => return "ADV-S".to_string(),
        CONJUNCTION => {
            if is_crasis(parsed) {
                return "CONJ-K".to_string();
            }
            if is_negative(parsed) {
                return "CONJ-N".to_string();
            }
            return "CONJ".to_string();
        }
        CONDITIONAL => {
            if is_crasis(parsed) {
                return "COND-K".to_string();
            }
            return "COND".to_string();
        }
        PARTICLE => {
            if is_negative(parsed) {
                return "PRT-N".to_string();
            }
            if is_interrogative(parsed) {
                return "PRT-I".to_string();
            }
            return "PRT".to_string();
        }
        PREPOSITION => return "PREP".to_string(),
        INTERJECTION => return "INJ".to_string(),
        ARAMAIC_TRANSLITERATION => return "ARAM".to_string(),
        HEBREW_TRANSLITERATION => return "HEB".to_string(),
        PROPER_NOUN => {
            if is_indeclinable(parsed) && case(parsed) == 0 {
                return "N-PRI".to_string();
            }
        }
        NUMERAL => {
            if is_indeclinable(parsed) {
                return "A-NUI".to_string();
            }
        }
        LETTER => {
            if is_indeclinable(parsed) {
                return "N-LI".to_string();
            }
        }
        NOUN => {
            if is_indeclinable(parsed) {
                return "N-OI".to_string();
            }
        }
        _ => {}
    }

    match pos {
        VERB => return vp_string(String::from("V"), parsed),
        NOUN => return cng_string(String::from("N"), parsed),
        ARTICLE => return cng_string(String::from("T"), parsed),
        ADJECTIVE => return cng_string(String::from("A"), parsed),
        RELATIVE_PRONOUN => return cng_string(String::from("R"), parsed),
        RECIPROCAL_PRONOUN => return cng_string(String::from("C"), parsed),
        DEMONSTRATIVE_PRONOUN => return cng_string(String::from("D"), parsed),
        REFLEXIVE_PRONOUN => return cng_string(fst_string(String::from("F-"), parsed), parsed),
        POSSESSIVE_PRONOUN => return cng_string(fs_ref_string(String::from("S-"), parsed), parsed),
        PERSONAL_PRONOUN => return pcn_string(String::from("P"), parsed),
        PROPER_NOUN => {
            if is_indeclinable(parsed) {
                return cng_string(String::from("IPN"), parsed);
            }
            return cng_string(String::from("PN"), parsed);
        }
        PRONOUN => {
            if is_correlative(parsed) && is_interrogative(parsed) {
                return cng_string(String::from("Q"), parsed);
            }
            if is_correlative(parsed) {
                return cng_string(String::from("K"), parsed);
            }
            if is_interrogative(parsed) {
                return cng_string(String::from("I"), parsed);
            }
            if is_indefinite(parsed) {
                return cng_string(String::from("X"), parsed);
            }
            return cng_string(String::from("O"), parsed);
        }
        SUPERLATIVE_NOUN => {
            let mut s = cng_string(String::from("N"), parsed);
            s.push_str("-S");
            return s;
        }
        SUPERLATIVE_ADJECTIVE => {
            let mut s = cng_string(String::from("A"), parsed);
            s.push_str("-S");
            return s;
        }
        COMPARATIVE_NOUN => {
            let mut s = cng_string(String::from("N"), parsed);
            s.push_str("-C");
            return s;
        }
        COMPARATIVE_ADJECTIVE => {
            let mut s = cng_string(String::from("A"), parsed);
            s.push_str("-C");
            return s;
        }
        _ => {}
    }

    "".to_string()
}

fn vp(mut parsing: u32, code: &[char]) -> Result<u32, ParseError> {
    if code.is_empty() {
        return Err(ParseError::Incomplete(parsing));
    }

    let mut index = 0;

    if code[index] == '-' {
        index += 1;
        if index >= code.len() {
            return Err(ParseError::Incomplete(parsing));
        }
    }

    if code[index] == '2' {
        index += 1;
        if index >= code.len() {
            return Err(ParseError::UnknownTenseForm(parsing, '2'));
        }

        parsing |= match code[index] {
            'F' | 'f' => SECOND_FUTURE,
            'A' | 'a' => SECOND_AORIST,
            'R' | 'r' => SECOND_PERFECT,
            'L' | 'l' => SECOND_PLUPERFECT,
            _ => {
                return Err(ParseError::UnknownTenseForm(parsing, code[index]));
            }
        }
    } else {
        parsing |= match code[index] {
            'P' | 'p' => PRESENT,
            'I' | 'i' => IMPERFECT,
            'F' | 'f' => FUTURE,
            'A' | 'a' => AORIST,
            'R' | 'r' => PERFECT,
            'L' | 'l' => PLUPERFECT,
            _ => {
                return Err(ParseError::UnknownTenseForm(parsing, code[index]));
            }
        }
    }

    index += 1;
    if index >= code.len() {
        return Err(ParseError::Incomplete(parsing));
    }

    parsing |= match code[index] {
        'A' | 'a' => ACTIVE_VOICE,
        'M' | 'm' => MIDDLE_VOICE,
        'P' | 'p' => PASSIVE_VOICE,
        'E' | 'e' => MIDDLE_PASSIVE_VOICE,
        'D' | 'd' => MIDDLE_DEPONENT_VOICE,
        'O' | 'o' => PASSIVE_DEPONENT_VOICE,
        'N' | 'n' => MIDDLE_PASSIVE_DEPONENT_VOICE,
        _ => {
            return Err(ParseError::UnknownVoice(parsing, code[index]));
        }
    };

    index += 1;
    if index >= code.len() {
        return Err(ParseError::Incomplete(parsing));
    }

    let pos = match code[index] {
        'I' | 'i' => INDICATIVE_MOOD,
        'S' | 's' => SUBJUNCTIVE_MOOD,
        'O' | 'o' => OPTATIVE_MOOD,
        'M' | 'm' => IMPERATIVE_MOOD,
        'N' | 'n' => INFINITIVE_MOOD,
        'P' | 'p' => PARTICIPLE_MOOD,
        _ => {
            return Err(ParseError::UnknownMood(parsing, code[index]));
        }
    };
    parsing |= pos;

    index += 1;
    if index < code.len() && code[index] == '-' {
        index += 1;
    }

    // Different Verb types require different ending types

    let remaining = code.len() - index;
    if remaining == 2 {
        parsing |= fst(code[index]);
        parsing |= n(code[index + 1]);
        return Ok(parsing);
    } else if remaining >= 3 {
        return cng(parsing, &code[index..]);
    } else if remaining == 0 && pos == INFINITIVE_MOOD {
        return Ok(parsing);
    }

    Err(ParseError::Incomplete(parsing))
}

fn vp_string(mut s: String, parsed: u32) -> String {
    match tense(parsed) {
        SECOND_FUTURE => s.push_str("-2F"),
        SECOND_AORIST => s.push_str("-2A"),
        SECOND_PERFECT => s.push_str("-2R"),
        SECOND_PLUPERFECT => s.push_str("-2L"),
        PRESENT => s.push_str("-P"),
        IMPERFECT => s.push_str("-I"),
        FUTURE => s.push_str("-F"),
        AORIST => s.push_str("-A"),
        PERFECT => s.push_str("-R"),
        PLUPERFECT => s.push_str("-L"),
        _ => {
            return s;
        }
    }

    match voice(parsed) {
        ACTIVE_VOICE => s.push('A'),
        MIDDLE_VOICE => s.push('M'),
        PASSIVE_VOICE => s.push('P'),
        MIDDLE_PASSIVE_VOICE => s.push('E'),
        MIDDLE_DEPONENT_VOICE => s.push('D'),
        PASSIVE_DEPONENT_VOICE => s.push('O'),
        MIDDLE_PASSIVE_DEPONENT_VOICE => s.push('N'),
        _ => {
            return s;
        }
    }

    match mood(parsed) {
        INDICATIVE_MOOD => s.push('I'),
        SUBJUNCTIVE_MOOD => s.push('S'),
        OPTATIVE_MOOD => s.push('O'),
        IMPERATIVE_MOOD => s.push('M'),
        INFINITIVE_MOOD => s.push('N'),
        PARTICIPLE_MOOD => s.push('P'),
        _ => {
            return s;
        }
    };

    if mood(parsed) == PARTICIPLE_MOOD {
        return cng_string(s, parsed);
    }

    match person(parsed) {
        FIRST_PERSON => s.push_str("-1"),
        SECOND_PERSON => s.push_str("-2"),
        THIRD_PERSON => s.push_str("-3"),
        _ => {
            return s;
        }
    }

    match number(parsed) {
        SINGULAR => s.push('S'),
        PLURAL => s.push('P'),
        _ => {
            return s;
        }
    }

    s
}

fn n(code: char) -> u32 {
    match code {
        'S' | 's' | '1' => SINGULAR,
        'P' | 'p' | '2' => PLURAL,
        _ => UNKNOWN,
    }
}

fn ref_n(code: char) -> u32 {
    match code {
        'S' | 's' | '1' => REF_SINGULAR,
        'P' | 'p' | '2' => REF_PLURAL,
        _ => UNKNOWN,
    }
}

fn cng_string(mut s: String, parsed: u32) -> String {
    let pos = part_of_speech(parsed);
    let dash = pos != POSSESSIVE_PRONOUN && pos != REFLEXIVE_PRONOUN;

    match case(parsed) {
        NOMINATIVE => {
            if dash {
                s.push('-');
            }
            s.push('N');
        }
        ACCUSATIVE => {
            if dash {
                s.push('-');
            }
            s.push('A');
        }
        GENITIVE => {
            if dash {
                s.push('-');
            }
            s.push('G');
        }
        DATIVE => {
            if dash {
                s.push('-');
            }
            s.push('D');
        }
        VOCATIVE => {
            if dash {
                s.push('-');
            }
            s.push('V');
        }
        _ => return s,
    }

    match number(parsed) {
        SINGULAR => {
            s.push('S');
        }
        PLURAL => {
            s.push('P');
        }
        _ => {
            return s;
        }
    }

    match gender(parsed) {
        MASCULINE => {
            s.push('M');
        }
        FEMININE => {
            s.push('F');
        }
        NEUTER => {
            s.push('N');
        }
        _ => {
            //    This library recognises U but do not produce it.
            //    s.push_str("U");
        }
    };

    if is_crasis(parsed) {
        s.push_str("-K");
    }

    if is_negative(parsed) {
        s.push_str("-N");
    }

    s
}

fn cng(mut parsing: u32, code: &[char]) -> Result<u32, ParseError> {
    if code.is_empty() {
        return Err(ParseError::Incomplete(parsing));
    }

    let mut index = 0;

    if code[index] == '-' {
        index += 1;
        if index >= code.len() {
            return Err(ParseError::Incomplete(parsing));
        }
    }

    match code[index] {
        'N' | 'n' => parsing |= NOMINATIVE,
        'A' | 'a' => parsing |= ACCUSATIVE,
        'G' | 'g' => parsing |= GENITIVE,
        'D' | 'd' => parsing |= DATIVE,
        'V' | 'v' => parsing |= VOCATIVE,
        _ => return Err(ParseError::UnknownCase(parsing, code[index])),
    }

    index += 1;
    if index >= code.len() {
        return Err(ParseError::Incomplete(parsing));
    }

    match code[index] {
        'S' | 's' | '1' => parsing |= SINGULAR,
        'P' | 'p' | '2' => parsing |= PLURAL,
        _ => {
            return Err(ParseError::UnknownPerson(parsing, code[index]));
        }
    }

    index += 1;
    if index >= code.len() {
        match part_of_speech(parsing) {
            NOUN | ADJECTIVE | PROPER_NOUN | PRONOUN => return Ok(parsing),
            _ => return Err(ParseError::Incomplete(parsing)),
        }
    }

    match code[index] {
        'M' | 'm' => parsing |= MASCULINE,
        'F' | 'f' => parsing |= FEMININE,
        'N' | 'n' => parsing |= NEUTER,
        'U' | 'u' | '-' => parsing |= 0,
        _ => {
            return Err(ParseError::UnknownGender(parsing, code[index]));
        }
    };

    if code[index] != '-' {
        index += 1;
    }
    if index >= code.len() {
        return Ok(parsing);
    }
    if code[index] != '-' && code[index] != ' ' {
        return Err(ParseError::UnexpectedCharacter(parsing, code[index], index));
    }

    index += 1;
    if index >= code.len() {
        return Ok(parsing);
    }

    let pos = part_of_speech(parsing);
    if code[index] == 'S' || code[index] == 's' {
        if pos == ADJECTIVE {
            return Ok(set_part_of_speech(parsing, SUPERLATIVE_ADJECTIVE));
        }
        if pos == ADVERB {
            return Ok(set_part_of_speech(parsing, SUPERLATIVE_ADVERB));
        }
        if pos == NOUN {
            return Ok(set_part_of_speech(parsing, SUPERLATIVE_NOUN));
        }
    } else if code[index] == 'C' || code[index] == 'c' {
        if pos == ADJECTIVE {
            return Ok(set_part_of_speech(parsing, COMPARATIVE_ADJECTIVE));
        }
        if pos == ADVERB {
            return Ok(set_part_of_speech(parsing, COMPARATIVE_ADVERB));
        }
        if pos == NOUN {
            return Ok(set_part_of_speech(parsing, COMPARATIVE_NOUN));
        }
    } else if code[index] == 'K' || code[index] == 'k' {
        return Ok(parsing | CRASIS);
    } else if code[index] == 'N' || code[index] == 'n' {
        return Ok(parsing | NEGATIVE);
    }

    Err(ParseError::UnexpectedCharacter(parsing, code[index], index))
}

fn fst_string(mut s: String, parsed: u32) -> String {
    match person(parsed) {
        FIRST_PERSON => s.push('1'),
        SECOND_PERSON => s.push('2'),
        THIRD_PERSON => s.push('3'),
        _ => return s,
    }
    s
}

fn fs_ref_string(mut s: String, parsed: u32) -> String {
    match (person(parsed), ref_number(parsed)) {
        (FIRST_PERSON, REF_SINGULAR) => s.push_str("1S"),
        (SECOND_PERSON, REF_SINGULAR) => s.push_str("2S"),
        (FIRST_PERSON, REF_PLURAL) => s.push_str("1P"),
        (SECOND_PERSON, REF_PLURAL) => s.push_str("2P"),
        _ => return s,
    }
    s
}

fn fst(code: char) -> u32 {
    match code {
        '1' => FIRST_PERSON,
        '2' => SECOND_PERSON,
        '3' => THIRD_PERSON,
        _ => UNKNOWN,
    }
}

fn pcn_string(mut s: String, parsed: u32) -> String {
    match person(parsed) {
        FIRST_PERSON => s.push_str("-1"),
        SECOND_PERSON => s.push_str("-2"),
        _ => return cng_string(s, parsed),
    }

    match case(parsed) {
        NOMINATIVE => s.push('N'),
        ACCUSATIVE => s.push('A'),
        GENITIVE => s.push('G'),
        DATIVE => s.push('D'),
        VOCATIVE => s.push('V'),
        _ => {
            return s;
        }
    }

    match number(parsed) {
        //'S' | 's' | '1' => return r | SINGULAR,
        //'P' | 'p' | '2' => return r | PLURAL,
        SINGULAR => s.push('S'),
        PLURAL => s.push('P'),
        _ => {}
    }

    if is_crasis(parsed) {
        s.push_str("-K");
    }

    s
}

fn pcn(mut parsing: u32, code: &[char]) -> Result<u32, ParseError> {
    let mut index = 0;

    if code.is_empty() {
        return Err(ParseError::Incomplete(parsing));
    }

    if code[index] == '-' {
        index += 1;
        if index >= code.len() {
            return Err(ParseError::Incomplete(parsing));
        }
    }

    match code[index] {
        '1' => parsing |= FIRST_PERSON,
        '2' => parsing |= SECOND_PERSON,
        _ => return cng(parsing, code),
    }

    index += 1;
    if index >= code.len() {
        return Err(ParseError::Incomplete(parsing));
    }

    match code[index] {
        'N' | 'n' => parsing |= NOMINATIVE,
        'A' | 'a' => parsing |= ACCUSATIVE,
        'G' | 'g' => parsing |= GENITIVE,
        'D' | 'd' => parsing |= DATIVE,
        'V' | 'v' => parsing |= VOCATIVE,
        _ => return Err(ParseError::UnknownCase(parsing, code[index])),
    }

    index += 1;
    if index >= code.len() {
        return Err(ParseError::Incomplete(parsing));
    }

    match code[index] {
        'S' | 's' | '1' => parsing |= SINGULAR,
        'P' | 'p' | '2' => parsing |= PLURAL,
        _ => return Err(ParseError::UnknownNumber(parsing, code[index])),
    }

    index += 1;
    if index >= code.len() {
        return Ok(parsing);
    }
    if code[index] != '-' && code[index] != ' ' {
        return Err(ParseError::UnexpectedCharacter(parsing, code[index], index));
    }

    index += 1;
    if index >= code.len() {
        return Ok(parsing);
    }

    if code[index] == 'K' || code[index] == 'k' {
        return Ok(parsing | CRASIS);
    }

    Err(ParseError::UnexpectedCharacter(parsing, code[index], index))
}

// pos_to_string returns a capitalised English name for the part of
// speech with spaces between words.
pub fn pos_to_string(parsing: u32) -> &'static str {
    let pos = part_of_speech(parsing);
    return match pos {
        UNKNOWN => "",
        PARTICLE => {
            if is_interrogative(parsing) {
                return "Interrogative Particle";
            }
            "Particle"
        }
        VERB => "Verb",
        NOUN => "Noun",
        ADJECTIVE => "Adjective",
        ADVERB => "Adverb",
        CONJUNCTION => "Conjunction",
        PROPER_NOUN => {
            if is_interrogative(parsing) {
                return "Interrogative Proper Noun";
            }
            if is_indeclinable(parsing) {
                return "Indeclinable Proper Noun";
            }
            "Proper Noun"
        }
        PREPOSITION => "Preposition",
        CONDITIONAL => "Conditional",
        ARTICLE => "Definite Article",
        INTERJECTION => "Interjection",
        PRONOUN => {
            if is_interrogative(parsing) {
                return "Interrogative Pronoun";
            }
            if is_indefinite(parsing) {
                return "Indefinite Pronoun";
            }
            "Pronoun"
        }
        PERSONAL_PRONOUN => "Personal Pronoun",
        RELATIVE_PRONOUN => "Relative Pronoun",
        RECIPROCAL_PRONOUN => "Reciprocal Pronoun",
        DEMONSTRATIVE_PRONOUN => "Demonstrative Pronoun",
        REFLEXIVE_PRONOUN => "Reflexive Pronoun",
        POSSESSIVE_PRONOUN => "Posessive Pronoun",
        SUPERLATIVE_NOUN => "Superlative Noun",
        SUPERLATIVE_ADJECTIVE => "Superlative Adjective",
        COMPARATIVE_NOUN => "Comparative Noun",
        COMPARATIVE_ADJECTIVE => "Comparative Adjective",
        TRANSLITERATION => "Transliteration",
        HEBREW_TRANSLITERATION => "Hebrew Transliteration",
        ARAMAIC_TRANSLITERATION => "Aramaic Transliteration",
        NUMERAL => "Numeral",
        LETTER => "Letter",
        _ => "",
    };
}

// pos_to_camel_case returns a capitalised English name for the part of
// speech with no spaces between words.
pub fn pos_to_camel_case(parsing: u32) -> &'static str {
    let pos = part_of_speech(parsing);
    return match pos {
        UNKNOWN => "Unknown",
        PARTICLE => {
            if is_interrogative(parsing) {
                return "InterrogativeParticle";
            }
            "Particle"
        }
        VERB => "Verb",
        NOUN => "Noun",
        ADJECTIVE => "Adjective",
        ARTICLE => "DefiniteArticle",
        ADVERB => "Adverb",
        PRONOUN => {
            if is_interrogative(parsing) {
                return "InterrogativePronoun";
            }
            if is_indefinite(parsing) {
                return "IndefinitePronoun";
            }
            "Pronoun"
        }
        PREPOSITION => "Preposition",
        CONJUNCTION => "Conjunction",
        CONDITIONAL => "Conditional",
        INTERJECTION => "Interjection",
        RELATIVE_PRONOUN => "RelativePronoun",
        RECIPROCAL_PRONOUN => "ReciprocalPronoun",
        DEMONSTRATIVE_PRONOUN => "DemonstrativePronoun",
        REFLEXIVE_PRONOUN => "ReflexivePronoun",
        POSSESSIVE_PRONOUN => "PosessivePronoun",
        PERSONAL_PRONOUN => "PersonalPronoun",
        PROPER_NOUN => {
            if is_interrogative(parsing) {
                return "InterrogativeProperNoun";
            }
            if is_indeclinable(parsing) {
                return "IndeclinableProperNoun";
            }
            "ProperNoun"
        }
        SUPERLATIVE_NOUN => "SuperlativeNoun",
        SUPERLATIVE_ADJECTIVE => "SuperlativeAdjective",
        COMPARATIVE_NOUN => "ComparativeNoun",
        COMPARATIVE_ADJECTIVE => "ComparativeAdjective",
        TRANSLITERATION => "HebrewTransliteration",
        HEBREW_TRANSLITERATION => "HebrewTransliteration",
        ARAMAIC_TRANSLITERATION => "AramaicTransliteration",
        NUMERAL => "Numeral",
        LETTER => "Letter",
        _ => "",
    };
}

pub fn string_to_pos(text: &str) -> u32 {
    return match text.to_lowercase().as_str() {
        "verb" => VERB,
        "noun" => NOUN,
        "article" | "definitearticle" => ARTICLE,
        "adverb" => ADVERB,
        "adjective" => ADJECTIVE,
        "pronoun" => PRONOUN,
        "particle" => PARTICLE,
        "preposition" => PREPOSITION,
        "conjunction" => CONJUNCTION,
        "conditional" => CONDITIONAL,
        "interjection" => INTERJECTION,
        "relative pronoun" | "relativepronoun" => RELATIVE_PRONOUN,
        "interrogative pronoun" | "interrogativepronoun" => PRONOUN | INTERROGATIVE,
        "indefinite pronoun" | "indefinitepronoun" => PRONOUN | INDEFINITE,
        "reciprocal pronoun" | "reciprocalpronoun" => RECIPROCAL_PRONOUN,
        "demonstrative pronoun" | "demonstrativepronoun" => DEMONSTRATIVE_PRONOUN,
        "reflexive pronoun" | "reflexivepronoun" => REFLEXIVE_PRONOUN,
        "posessive pronoun" | "posessivepronoun" => POSSESSIVE_PRONOUN,
        "personal pronoun" | "personalpronoun" => PERSONAL_PRONOUN,
        "indeclinable proper noun" | "indeclinablepropernoun" => PROPER_NOUN | INDECLINABLE,
        "proper noun" | "propernoun" => PROPER_NOUN,
        "superlative noun" | "superlativenoun" => SUPERLATIVE_NOUN,
        "superlative adjective" | "superlativeadjective" => SUPERLATIVE_ADJECTIVE,
        "comparative noun" | "comparativenoun" => COMPARATIVE_NOUN,
        "comparative adjective" | "comparativeadjective" => COMPARATIVE_ADJECTIVE,
        "transliteration" => TRANSLITERATION,
        "hebrew transliteration" | "hebrewtransliteration" => HEBREW_TRANSLITERATION,
        "aramaic transliteration" | "aramaictransliteration" => ARAMAIC_TRANSLITERATION,
        "interrogative particle" | "interrogativeparticle" => INTERROGATIVE | PARTICLE,
        "letter" => LETTER,
        "numeral" => NUMERAL,
        _ => 0,
    };
}

#[cfg(test)]
mod tests {
    use crate::parse::*;
    use crate::string::*;
    use crate::test_case;

    #[test]
    fn test_to_string() {
        assert_eq!(to_string(NOUN), "N", "failed");
        assert_eq!(to_string(CONDITIONAL | CRASIS), "COND-K", "failed");
        assert_eq!(to_string(ARTICLE | NOMINATIVE), "T-N", "failed");
        assert_eq!(to_string(ARTICLE | GENITIVE), "T-G", "failed");
        assert_eq!(to_string(ARTICLE | GENITIVE | SINGULAR), "T-GS", "failed");
        assert_eq!(
            to_string(ARTICLE | GENITIVE | SINGULAR | NEUTER),
            "T-GSN",
            "failed"
        );
        assert_eq!(
            to_string(ADJECTIVE | ACCUSATIVE | PLURAL | MASCULINE),
            "A-APM",
            "failed"
        );
        assert_eq!(
            to_string(INDECLINABLE | PROPER_NOUN | NOMINATIVE | SINGULAR),
            "IPN-NS",
            "failed"
        );
        assert_eq!(to_string(NOUN | NOMINATIVE | SINGULAR), "N-NS", "failed");
        assert_eq!(to_string(NOUN | GENITIVE | SINGULAR), "N-GS", "failed");
        assert_eq!(
            to_string(PROPER_NOUN | ACCUSATIVE | PLURAL | MASCULINE),
            "PN-APM",
            "failed"
        );
        assert_eq!("ADV-S", to_string(SUPERLATIVE_ADVERB), "failed");
        assert_eq!("ADV-C", to_string(COMPARATIVE_ADVERB), "failed");
        assert_eq!(
            to_string(PERSONAL_PRONOUN | FIRST_PERSON | ACCUSATIVE | SINGULAR),
            "P-1AS",
            "failed"
        );
        assert_eq!(
            to_string(
                POSSESSIVE_PRONOUN | SECOND_PERSON | REF_SINGULAR | ACCUSATIVE | PLURAL | MASCULINE
            ),
            "S-2SAPM",
            "expected 'S-2SAPM'"
        );
        assert_eq!(
            to_string(
                POSSESSIVE_PRONOUN | SECOND_PERSON | REF_PLURAL | ACCUSATIVE | PLURAL | MASCULINE
            ),
            "S-2PAPM",
            "failed"
        );
        assert_eq!(
            to_string(REFLEXIVE_PRONOUN | THIRD_PERSON | ACCUSATIVE | PLURAL | MASCULINE),
            "F-3APM",
            "failed"
        );
        assert_eq!(
            to_string(DEMONSTRATIVE_PRONOUN | ACCUSATIVE | PLURAL | MASCULINE | CRASIS),
            "D-APM-K",
            "failed"
        );
        assert_eq!(
            to_string(VERB | AORIST | ACTIVE_VOICE | INDICATIVE_MOOD | FIRST_PERSON | PLURAL),
            "V-AAI-1P",
            "failed"
        );
        assert_eq!(
            to_string(
                VERB | SECOND_AORIST | ACTIVE_VOICE | INDICATIVE_MOOD | FIRST_PERSON | PLURAL
            ),
            "V-2AAI-1P",
            "failed"
        );
        assert_eq!(
            to_string(
                VERB | SECOND_AORIST
                    | ACTIVE_VOICE
                    | PARTICIPLE_MOOD
                    | ACCUSATIVE
                    | PLURAL
                    | FEMININE
            ),
            "V-2AAP-APF",
            "failed"
        );
    }

    #[test]
    fn test_basic_parse() {
        assert_eq!(
            from_string("N").expect_err("incomplete"),
            ParseError::Incomplete(NOUN),
            "failed"
        );
        assert_eq!(
            from_string("n").expect_err("incomplete"),
            ParseError::Incomplete(NOUN),
            "failed"
        );
        assert_eq!(
            from_string("COND-K").expect("success"),
            CONDITIONAL | CRASIS,
            "failed"
        );
        assert_eq!(
            from_string("T-G").expect_err("incomplete"),
            ParseError::Incomplete(ARTICLE | GENITIVE),
            "failed"
        );
        assert_eq!(
            from_string("T-GS").expect_err("parse failed"),
            ParseError::Incomplete(ARTICLE | GENITIVE | SINGULAR),
            "failed"
        );
        assert_eq!(
            from_string("N-GS").expect("parse failed"),
            NOUN | GENITIVE | SINGULAR,
            "failed"
        );
        assert_eq!(
            from_string("A-GS").expect("parse failed"),
            ADJECTIVE | GENITIVE | SINGULAR,
            "failed"
        );
        assert_eq!(
            from_string("A-G").expect_err("incomplete"),
            ParseError::Incomplete(ADJECTIVE | GENITIVE),
            "failed"
        );
        assert_eq!(
            from_string("A-GS").expect("parse failed"),
            ADJECTIVE | GENITIVE | SINGULAR,
            "failed"
        );
        assert_eq!(
            from_string("V-AAI-1P").expect("parse failed"),
            VERB | AORIST | ACTIVE_VOICE | INDICATIVE_MOOD | FIRST_PERSON | PLURAL,
            "failed"
        );
        assert_eq!(
            from_string("T-GSN").expect("parse failed"),
            ARTICLE | GENITIVE | SINGULAR | NEUTER,
            "failed"
        );
        assert_eq!(
            from_string("A-APM").expect("parse failed"),
            ADJECTIVE | ACCUSATIVE | PLURAL | MASCULINE,
            "failed"
        );
        assert_eq!(
            from_string("PN-NSM").expect("parse failed"),
            PROPER_NOUN | NOMINATIVE | SINGULAR | MASCULINE,
            "failed"
        );

        let t = from_string("A-APF-C").expect("parse failed");
        assert_eq!(
            t,
            COMPARATIVE_ADJECTIVE | ACCUSATIVE | PLURAL | FEMININE,
            "failed on 'A-APF-C', pos: {} {} {:#018b}--{:#018b}",
            part_of_speech(t),
            part_of_speech(t),
            from_string("A-APF-C").expect("parse"),
            (COMPARATIVE_ADJECTIVE | ACCUSATIVE | PLURAL | FEMININE),
        );
        assert_eq!(from_string("ADV").expect("parse"), ADVERB, "failed");
        assert_eq!(
            from_string("ADV-S").expect("parse"),
            SUPERLATIVE_ADVERB,
            "failed"
        );
        assert_eq!(
            from_string("ADV-C").expect("parse"),
            COMPARATIVE_ADVERB,
            "failed"
        );
        assert_eq!(
            from_string("P-1AS").expect("parse failed"),
            PERSONAL_PRONOUN | FIRST_PERSON | ACCUSATIVE | SINGULAR,
            "failed"
        );
        assert_eq!(
            from_string("S-2SAPM").expect("parse failed"),
            POSSESSIVE_PRONOUN | SECOND_PERSON | ACCUSATIVE | PLURAL | MASCULINE,
            "failed"
        );
        assert_eq!(
            from_string("F-3APM").expect("parse failed"),
            REFLEXIVE_PRONOUN | THIRD_PERSON | ACCUSATIVE | PLURAL | MASCULINE,
            "failed"
        );
        assert_eq!(
            from_string("V-2AAI-1P").expect("parse failed"),
            VERB | SECOND_AORIST | ACTIVE_VOICE | INDICATIVE_MOOD | FIRST_PERSON | PLURAL,
            "failed"
        );
    }

    #[test]
    fn test_parsing_parts() {
        let parsed = from_string("D-GPM-K").expect("parsing fail");
        assert_eq!(part_of_speech(parsed), DEMONSTRATIVE_PRONOUN);
        assert_eq!(case(parsed), GENITIVE);

        let parsed = from_string("D-GPM").expect("parsing fail");
        assert_eq!(part_of_speech(parsed), DEMONSTRATIVE_PRONOUN);
        assert_eq!(is_crasis(parsed), false);
        assert_eq!(case(parsed), GENITIVE);

        // It is valid to leave gender as unkown. In some text
        // the gender of a word is genuinely not known.
        match from_string("T-NS") {
            Err(ParseError::Incomplete(parsed)) => {
                assert_eq!(part_of_speech(parsed), ARTICLE);
                assert_eq!(case(parsed), NOMINATIVE);
                assert_eq!(number(parsed), SINGULAR);
                assert_eq!(gender(parsed), UNKNOWN);
            }
            _ => assert!(false, "failed"),
        }

        match from_string("N-NSU") {
            Ok(parsed) => {
                assert_eq!(part_of_speech(parsed), NOUN);
                assert_eq!(case(parsed), NOMINATIVE);
                assert_eq!(number(parsed), SINGULAR);
                assert_eq!(gender(parsed), UNKNOWN);
            }
            Err(e) => assert!(false, "failed: {:?}", e),
        }

        match from_string("D-APM-K") {
            Ok(parsed) => {
                assert_eq!(part_of_speech(parsed), DEMONSTRATIVE_PRONOUN);
                assert_eq!(case(parsed), ACCUSATIVE);
                assert_eq!(number(parsed), PLURAL);
                assert_eq!(gender(parsed), MASCULINE);
                assert_eq!(is_crasis(parsed), true);
                assert_eq!(
                    to_string(DEMONSTRATIVE_PRONOUN | ACCUSATIVE | PLURAL | NEUTER | CRASIS),
                    "D-APN-K",
                    "failed"
                );
            }
            Err(e) => assert!(false, "failed: {:?}", e),
        }
    }

    #[test]
    fn test_parsing_nestle_file() {
        let file = test_case!("nestle-parsing.txt");
        let data = std::fs::read_to_string(file);
        if data.is_err() {
            assert_eq!(data.is_err(), false, "data file missing. {}", file);
        }
        let data = data.unwrap();
        for line in data.split("\n") {
            if line == "V-APS2P" || line == "form_morph" || line == "strongs" {
                // typo in data
                continue;
            }
            if line.ends_with("-ATT") {
                continue;
            }
            if line == "CONJ-P" {
                continue;
            }
            match from_string(line) {
                Ok(parsed) => {
                    let stringed = to_string(parsed);
                    assert_eq!(
                        line, stringed,
                        "loaded parsing {} but printed {}",
                        line, stringed
                    );
                }
                Err(e) => assert!(false, "Parsing {} failed. {:?}", line, e),
            }
        }
    }

    #[test]
    fn test_parsing_byz_file() {
        let file = test_case!("byz-parsing.txt");
        let data = std::fs::read_to_string(file);
        if data.is_err() {
            assert_eq!(data.is_err(), false, "data file missing. {}", file);
        }
        let data = data.unwrap();
        for line in data.split("\n") {
            if line.ends_with("-ABB") {
                continue;
            }
            if line.ends_with("-ATT") {
                continue;
            }
            if line == "CONJ-P" {
                continue;
            }
            match from_string(line) {
                Ok(parsed) => {
                    let stringed = to_string(parsed);
                    assert_eq!(
                        line, stringed,
                        "loaded parsing {} but printed {}",
                        line, stringed
                    );
                }
                Err(e) => assert!(false, "Parsing {} failed. {:?}", line, e),
            }
            //if part_of_speech(parsed) == COMPARATIVE_ADVERB {
            //   continue;
            //}
        }
    }

    #[test]
    fn test_string_parse() {
        assert_eq!(
            INDECLINABLE | PROPER_NOUN,
            string_to_pos("IndeclinableProperNoun")
        );
        assert_eq!(
            INDECLINABLE | PROPER_NOUN,
            string_to_pos("Indeclinable Proper Noun")
        );
        assert_eq!(
            pos_to_camel_case(INDECLINABLE | PROPER_NOUN),
            "IndeclinableProperNoun"
        );
        assert_eq!(
            pos_to_string(INDECLINABLE | PROPER_NOUN),
            "Indeclinable Proper Noun"
        );
    }
}
