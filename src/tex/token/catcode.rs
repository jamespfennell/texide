//! Data structures representing category codes and operations on them.
//!
//! The following table lists all 16 category codes in TeX. Names marked with * are never
//! returned from the lexer; instead, they are transformed into other category codes
//! or ignored.
//!
//! | name           | #  | e.g. | description |
//! |----------------|----|------|-------------|
//! | `Escape`*      | 0  | `\`  | Denotes the beginning of a control sequence.
//! | `BeginGroup`   | 1  | `{`  | Starts a new group/scope.
//! | `EndGroup`     | 2  | `}`  | Ends an existing new group/scope.
//! | `MathShift`    | 3  |      |
//! | `AlignmentTab` | 4  |      |
//! | `EndOfLine`*   | 5  | `\n` | New line in the input. Two consecutive new lines modulo whitespace create a `\par` control sequence.
//! | `Parameter`    | 6  | `#`  | Denotes the beginning of a parameter number; must generally be followed by a digit.
//! | `Superscript`  | 7  | `^`  | Puts following character or group in a superscript.
//! | `Subscript`    | 8  | `_`  | Puts following character or group in a subscript.
//! | `Ignored`*     | 9  |      | Ignored by the lexer.
//! | `Space`        | 10 | ` `  | Whitespace.
//! | `Letter`       | 11 | `A`  | A character that can be used as a control sequence name.
//! | `Other`        | 12 | `@`  | A character than cannot be used as a control sequence name.
//! | `Active`       | 13 |      |
//! | `Comment`*     | 14 | `%`  | Denotes the beginning of a comment; all remaining characters on the line will be ignored.
//! | `Invalid`*     | 15 |      | An invalid character; if this is read in the input, a error will fire.
//!
use crate::datastructures::scopedmap::ScopedMap;

use CatCode::*;
use RawCatCode::*;

// Exercise 7.3 in the TeX book
/// Enum representing all 11 category codes that can be returned by the lexer.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CatCode {
    BeginGroup,
    EndGroup,
    MathShift,
    AlignmentTab,
    Parameter,
    Superscript,
    Subscript,
    Space,
    Letter,
    Other,
    Active,
}

impl CatCode {
    pub fn int(&self) -> u8 {
        match self {
            BeginGroup => 1,
            EndGroup => 2,
            MathShift => 3,
            AlignmentTab => 4,
            Parameter => 6,
            Superscript => 7,
            Subscript => 8,
            Space => 10,
            Letter => 11,
            Other => 12,
            Active => 13,
        }
    }

    pub fn from_int(int: u8) -> Option<CatCode> {
        match int {
            1 => Some(BeginGroup),
            2 => Some(EndGroup),
            3 => Some(MathShift),
            4 => Some(AlignmentTab),
            6 => Some(Parameter),
            7 => Some(Superscript),
            8 => Some(Subscript),
            10 => Some(Space),
            11 => Some(Letter),
            12 => Some(Other),
            13 => Some(Active),
            _ => None,
        }
    }
}

/// Enum representing all 16 category codes in TeX.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RawCatCode {
    Regular(CatCode),
    Escape,
    EndOfLine,
    Ignored,
    Comment,
    Invalid,
}

impl RawCatCode {
    pub fn int(&self) -> u8 {
        match self {
            Regular(cat_code) => cat_code.int(),
            Escape => 0,
            EndOfLine => 5,
            Ignored => 9,
            Comment => 14,
            Invalid => 15,
        }
    }

    pub fn from_int(int: u8) -> Option<RawCatCode> {
        match int {
            0 => Some(Escape),
            5 => Some(EndOfLine),
            9 => Some(Ignored),
            14 => Some(Comment),
            15 => Some(Invalid),
            int => CatCode::from_int(int).map(|cat_code| Regular(cat_code)),
        }
    }
}

pub fn or_default(c: Option<&RawCatCode>) -> RawCatCode {
    match c {
        None => Regular(Other),
        Some(&c) => c,
    }
}

// TODO: the cat code wrapper should have nice insert ops for regular catcodes
pub fn tex_defaults() -> ScopedMap<char, RawCatCode> {
    let mut cat_code_map = ScopedMap::new();
    set_tex_defaults(&mut cat_code_map);
    cat_code_map
}

pub fn set_tex_defaults(cat_code_map: &mut ScopedMap<char, RawCatCode>) {
    cat_code_map.extend(std::array::IntoIter::new([
        ('\\', Escape),
        ('{', Regular(BeginGroup)),
        ('}', Regular(EndGroup)),
        ('$', Regular(MathShift)),
        ('&', Regular(AlignmentTab)),
        ('\n', EndOfLine),
        ('#', Regular(Parameter)),
        ('^', Regular(Superscript)),
        ('_', Regular(Subscript)),
        ('~', Regular(Active)),
        ('%', Comment),
        //
        (' ', Regular(Space)), // TODO: other white space characters?
        //
        ('A', Regular(Letter)),
        ('B', Regular(Letter)),
        ('C', Regular(Letter)),
        ('D', Regular(Letter)),
        ('E', Regular(Letter)),
        ('F', Regular(Letter)),
        ('G', Regular(Letter)),
        ('H', Regular(Letter)),
        ('I', Regular(Letter)),
        ('J', Regular(Letter)),
        ('K', Regular(Letter)),
        ('L', Regular(Letter)),
        ('M', Regular(Letter)),
        ('N', Regular(Letter)),
        ('O', Regular(Letter)),
        ('P', Regular(Letter)),
        ('Q', Regular(Letter)),
        ('R', Regular(Letter)),
        ('S', Regular(Letter)),
        ('T', Regular(Letter)),
        ('U', Regular(Letter)),
        ('V', Regular(Letter)),
        ('W', Regular(Letter)),
        ('X', Regular(Letter)),
        ('Y', Regular(Letter)),
        ('Z', Regular(Letter)),
        //
        ('a', Regular(Letter)),
        ('b', Regular(Letter)),
        ('c', Regular(Letter)),
        ('d', Regular(Letter)),
        ('e', Regular(Letter)),
        ('f', Regular(Letter)),
        ('g', Regular(Letter)),
        ('h', Regular(Letter)),
        ('i', Regular(Letter)),
        ('j', Regular(Letter)),
        ('k', Regular(Letter)),
        ('l', Regular(Letter)),
        ('m', Regular(Letter)),
        ('n', Regular(Letter)),
        ('o', Regular(Letter)),
        ('p', Regular(Letter)),
        ('q', Regular(Letter)),
        ('r', Regular(Letter)),
        ('s', Regular(Letter)),
        ('t', Regular(Letter)),
        ('u', Regular(Letter)),
        ('v', Regular(Letter)),
        ('w', Regular(Letter)),
        ('x', Regular(Letter)),
        ('y', Regular(Letter)),
        ('z', Regular(Letter)),
    ]))
}

#[cfg(test)]
mod tests {
    use crate::tex::token::catcode::RawCatCode::Regular;
    use crate::tex::token::catcode::{CatCode, RawCatCode};

    fn all_cat_codes() -> Vec<CatCode> {
        vec![
            CatCode::BeginGroup,
            CatCode::EndGroup,
            CatCode::MathShift,
            CatCode::AlignmentTab,
            CatCode::Parameter,
            CatCode::Superscript,
            CatCode::Subscript,
            CatCode::Space,
            CatCode::Letter,
            CatCode::Other,
            CatCode::Active,
        ]
    }

    #[test]
    fn serialize_and_deserialize_raw_cat_code() {
        for cat_code in all_cat_codes() {
            assert_eq!(CatCode::from_int(cat_code.int()), Some(cat_code))
        }
    }

    #[test]
    fn serialize_and_deserialize_cat_code() {
        let mut all_raw_cat_codes = vec![
            RawCatCode::Escape,
            RawCatCode::EndOfLine,
            RawCatCode::Ignored,
            RawCatCode::Comment,
            RawCatCode::Invalid,
        ];
        for cat_code in all_cat_codes() {
            all_raw_cat_codes.push(Regular(cat_code))
        }
        for cat_code in all_raw_cat_codes {
            assert_eq!(RawCatCode::from_int(cat_code.int()), Some(cat_code))
        }
    }
}
