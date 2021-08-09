//! TeX execution driver.

use crate::tex::primitive;
use crate::tex::primitive::ExpansionPrimitive;
use crate::tex::state::TexState;
use crate::tex::token::stream;
use crate::tex::token::stream::{Stream, VecStream};
use crate::tex::token::token;

// TODO: accept a mutable reference to the state; we don't need to own it
pub fn run<S: TexState<S>>(state: S) -> anyhow::Result<S> {
    let mut input = ExpandedStream::<S> {
        unexpanded_stream: UnexpandedStream::<S> {
            s: state,
            stack: vec![],
        },
    };
    loop {
        match input.next()? {
            None => break,
            Some(token) => {
                // TODO: this is where the execution code goes
                println!("{:?}", token.value)
            }
        };
    }
    Ok(input.unexpanded_stream.s)
}

// TODO: maybe a better name?
struct UnexpandedStream<S> {
    s: S,
    stack: Vec<stream::VecStream>,
}

impl<S: TexState<S>> stream::Stream for UnexpandedStream<S> {
    fn next(&mut self) -> anyhow::Result<Option<token::Token>> {
        self.prepare_imut_peek()?;
        match self.stack.last_mut() {
            None => self.s.base_mut().input_module.next(),
            Some(top) => top.next(),
        }
    }

    fn prepare_imut_peek(&mut self) -> anyhow::Result<()> {
        loop {
            match self.stack.last_mut() {
                None => return self.s.base_mut().input_module.prepare_imut_peek(),
                Some(top) => match top.peek()? {
                    None => {
                        self.stack.pop();
                        continue;
                    }
                    Some(..) => return Ok(()),
                },
            }
        }
    }

    fn imut_peek(&self) -> anyhow::Result<Option<&token::Token>> {
        match self.stack.last() {
            None => self.s.base().input_module.imut_peek(),
            Some(top) => top.imut_peek(),
        }
    }
}

// TODO: Rename ExpandedInput
pub struct ExpandedStream<S> {
    unexpanded_stream: UnexpandedStream<S>,
}

impl<S: TexState<S>> stream::Stream for ExpandedStream<S> {
    fn next(&mut self) -> anyhow::Result<Option<token::Token>> {
        while self.expand_next()? {}
        self.unexpanded_stream.next()
    }

    fn prepare_imut_peek(&mut self) -> anyhow::Result<()> {
        while self.expand_next()? {}
        self.unexpanded_stream.prepare_imut_peek()
    }

    fn imut_peek(&self) -> anyhow::Result<Option<&token::Token>> {
        self.unexpanded_stream.imut_peek()
    }
}

impl<S: TexState<S>> ExpandedStream<S> {
    pub fn state(&self) -> &S {
        &self.unexpanded_stream.s
    }

    pub fn state_mut(&mut self) -> &mut S {
        &mut self.unexpanded_stream.s
    }

    pub fn stream(&mut self) -> &mut dyn Stream {
        self
    }

    pub fn unexpanded_stream(&mut self) -> &mut dyn Stream {
        &mut self.unexpanded_stream
    }

    pub fn expand_next(&mut self) -> anyhow::Result<bool> {
        self.unexpanded_stream.prepare_imut_peek()?;
        let command = match self.unexpanded_stream.imut_peek()? {
            None => None,
            Some(token) => match token.value {
                token::Value::Character(..) => None,
                token::Value::ControlSequence(_, ref name) => {
                    //println!("Considering command {}", name);
                    self.state().base().primitives.get(name)
                }
            },
        };
        let command = match command {
            Some(primitive::Primitive::Expansion(command)) => command.duplicate(),
            None => return Ok(false),
        };
        self.unexpanded_stream.consume()?;
        let output = command.call(self)?;
        self.unexpanded_stream.stack.push(output);
        Ok(true)
    }
}
