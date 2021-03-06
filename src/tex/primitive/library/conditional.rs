//! Conditional primitives

use crate::tex::primitive;
use crate::tex::primitive::ExpansionGeneric;

use crate::tex::state::TexState;
use crate::tex::token::stream;
use crate::tex::token::stream::Stream;
use crate::tex::token::token::Value;
use std::any;
use std::any::TypeId;

struct If;
struct Else;
struct Fi;

fn IfF<S: TexState<S>>(input: &mut primitive::Input<S>) -> anyhow::Result<stream::VecStream> {
    while let Some(token) = input.unexpanded_stream().next()? {
        if let Value::ControlSequence(_, name) = token.value {
            if let Some(c) = input.state().get_expansion_primitive(&name) {
                // TODO: switch on If, Else and Fi
                if Some(any::TypeId::of::<Else>()) == c.id() {
                    return Ok(stream::VecStream::new_empty());
                }
            }
        }
    }
    // TODO: end of the stream, ran out, should return an unexpected end of input error
    Ok(stream::VecStream::new_empty())
}

impl<S: TexState<S>> primitive::ExpansionGeneric<S> for If {
    fn call(&self, input: &mut primitive::Input<S>) -> anyhow::Result<stream::VecStream> {
        while let Some(token) = input.unexpanded_stream().next()? {
            if let Value::ControlSequence(_, name) = token.value {
                if let Some(c) = input.state().get_expansion_primitive(&name) {
                    // TODO: switch on If, Else and Fi
                    if Some(any::TypeId::of::<Else>()) == c.id() {
                        return Ok(stream::VecStream::new_empty());
                    }
                }
            }
        }
        // TODO: end of the stream, ran out, should return an unexpected end of input error
        Ok(stream::VecStream::new_empty())
    }
}

impl<State> primitive::ExpansionGeneric<State> for Else {
    fn call(&self, _: &mut primitive::Input<State>) -> anyhow::Result<stream::VecStream> {
        Ok(stream::VecStream::new(vec![]))
    }

    fn id(&self) -> Option<TypeId> {
        return Some(any::TypeId::of::<Else>());
    }
}

impl<State> primitive::ExpansionGeneric<State> for Fi {
    fn call(&self, _: &mut primitive::Input<State>) -> anyhow::Result<stream::VecStream> {
        Ok(stream::VecStream::new(vec![]))
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

pub fn get_else<State>() -> impl primitive::ExpansionGeneric<State> {
    return Else {};
}

pub fn get_fi<State>() -> impl primitive::ExpansionGeneric<State> {
    return Fi {};
}
