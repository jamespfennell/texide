///! Definition of the token type and related types
use crate::tex::token::catcode::CatCode;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Character(char, CatCode),
    ControlSequence(char, String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub value: Value,
    pub source: Option<Source>,
}

impl Token {
    pub fn new_letter(c: char) -> Token {
        return Token {
            value: Value::Character(c, CatCode::Letter),
            source: None,
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
