//! Conditional primitives

use crate::tex::primitive;
use crate::tex::primitive::ExpansionPrimitive;

use crate::tex::state::TexState;
use crate::tex::token::stream;
use crate::tex::token::stream::Stream;
use crate::tex::token::token::Value;
use std::any;
use std::any::TypeId;

struct If;
struct Else;
struct Fi;

fn IfF<S: TexState<S>>(input: &mut primitive::Input<S>) -> anyhow::Result<Box<dyn stream::Stream>> {
    while let Some(token) = input.unexpanded_stream().next()? {
        if let Value::ControlSequence(_, name) = token.value {
            if let Some(c) = input.state().get_expansion_primitive(&name) {
                // TODO: switch on If, Else and Fi
                if Some(any::TypeId::of::<Else>()) == c.id() {
                    return Ok(Box::new(stream::EmptyStream));
                }
            }
        }
    }
    // TODO: end of the stream, ran out, should return an unexpected end of input error
    Ok(Box::new(stream::EmptyStream))
}

impl<S: TexState<S>> primitive::ExpansionPrimitive<S> for If {
    fn call(&self, input: &mut primitive::Input<S>) -> anyhow::Result<Box<dyn stream::Stream>> {
        while let Some(token) = input.unexpanded_stream().next()? {
            if let Value::ControlSequence(_, name) = token.value {
                if let Some(c) = input.state().get_expansion_primitive(&name) {
                    // TODO: switch on If, Else and Fi
                    if Some(any::TypeId::of::<Else>()) == c.id() {
                        return Ok(Box::new(stream::EmptyStream));
                    }
                }
            }
        }
        // TODO: end of the stream, ran out, should return an unexpected end of input error
        Ok(Box::new(stream::EmptyStream))
    }
}

impl<State> primitive::ExpansionPrimitive<State> for Else {
    fn call(&self, _: &mut primitive::Input<State>) -> anyhow::Result<Box<dyn stream::Stream>> {
        Ok(Box::new(stream::VecStream::new(vec![])))
    }

    fn id(&self) -> Option<TypeId> {
        return Some(any::TypeId::of::<Else>());
    }
}

impl<State> primitive::ExpansionPrimitive<State> for Fi {
    fn call(&self, _: &mut primitive::Input<State>) -> anyhow::Result<Box<dyn stream::Stream>> {
        Ok(Box::new(stream::VecStream::new(vec![])))
    }
}

static IF_DOC: &str = "";

pub fn get_if<S: TexState<S>>() -> primitive::ExpansionStatic<S> {
    primitive::ExpansionStatic {
        call_fn: IfF,
        docs: IF_DOC,
        id: Some(any::TypeId::of::<Else>()),
    }
    //return If {};
}

pub fn get_else<State>() -> impl primitive::ExpansionPrimitive<State> {
    return Else {};
}

pub fn get_fi<State>() -> impl primitive::ExpansionPrimitive<State> {
    return Fi {};
}
