//! The Texide primitive, which returns the word Texide as 6 letter tokens.

use crate::tex::primitive;

use crate::tex::token::stream;

use crate::tex::token::token;

struct James {}

impl<State> primitive::ExpansionPrimitive<State> for James {
    fn call(&self, _: &mut primitive::Input<State>) -> anyhow::Result<Box<dyn stream::Stream>> {
        Ok(Box::new(stream::VecStream::new(vec![
            token::Token::new_letter('T'),
            token::Token::new_letter('e'),
            token::Token::new_letter('x'),
            token::Token::new_letter('i'),
            token::Token::new_letter('d'),
            token::Token::new_letter('e'),
        ])))
    }
}

pub fn get_texide<State>() -> impl primitive::ExpansionPrimitive<State> {
    return James {};
}

pub fn texide_command<State>(
    _: &mut primitive::Input<State>,
) -> anyhow::Result<Box<dyn stream::Stream>> {
    Ok(Box::new(stream::VecStream::new(vec![
        token::Token::new_letter('T'),
        token::Token::new_letter('e'),
        token::Token::new_letter('x'),
        token::Token::new_letter('i'),
        token::Token::new_letter('d'),
        token::Token::new_letter('e'),
    ])))
}
