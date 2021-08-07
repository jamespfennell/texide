//! The Texide primitive, which returns the word Texide as 6 letter tokens.

use crate::tex::primitive::expansion;

use crate::tex::token::stream;

use crate::tex::token::token;
use std::any::Any;

struct James {}

impl<State> expansion::Command<State> for James {
    fn run(&self, _: &mut dyn expansion::Input<State>) -> anyhow::Result<expansion::Output> {
        Ok(expansion::Output::Vec(stream::VecStream::new(vec![
            token::Token::new_letter('T'),
            token::Token::new_letter('e'),
            token::Token::new_letter('x'),
            token::Token::new_letter('i'),
            token::Token::new_letter('d'),
            token::Token::new_letter('e'),
        ])))
    }
}

static JAMES: James = James {};

pub fn get_texide<State>() -> impl expansion::Command<State> {
    return James {};
}

pub fn texide_command<State>(_: &mut dyn expansion::Input<State>) -> anyhow::Result<expansion::Output> {
    Ok(expansion::Output::Vec(stream::VecStream::new(vec![
        token::Token::new_letter('T'),
        token::Token::new_letter('e'),
        token::Token::new_letter('x'),
        token::Token::new_letter('i'),
        token::Token::new_letter('d'),
        token::Token::new_letter('e'),
    ])))
}