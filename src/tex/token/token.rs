///! Definition of the token type and related types
use crate::tex::token::catcode::CatCode;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    Character(char, CatCode),
    ControlSequence(char, String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub value: Value,
    pub source: Source,
}

impl Token {
    pub fn new_letter(c: char) -> Token {
        return Token {
            value: Value::Character(c, CatCode::Letter),
            // TODO: should not have to specify source or something
            // TODO: clearly we need to constructors for tokens
            // TODO: maybe even a nice constructor for VecStream?
            source: Source {
                line: Rc::new(Line {
                    content: "".to_string(),
                    line_number: 0,
                    file: Rc::new("".to_string()),
                }),
                position: 0,
            },
        };
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    pub content: String,
    pub line_number: isize,
    pub file: Rc<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Source {
    pub line: Rc<Line>,
    pub position: usize,
}
