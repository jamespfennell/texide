//! TeX execution driver.

use crate::tex::primitive;
use crate::tex::state;
use crate::tex::token::stream;
use crate::tex::token::token;

use crate::tex::token::stream::Stream;

use crate::tex::primitive::Input;
use crate::tex::state::TexState;
use std::convert::TryFrom;

pub trait StateAndStream {
    type State;

    fn state(&self) -> &Self::State;

    fn state_mut(&mut self) -> &mut Self::State;

    fn stream(&self) -> &dyn stream::Stream;

    fn stream_mut(&mut self) -> &mut dyn stream::Stream;
}

pub fn run<SAS>(state_and_stream: SAS)
where
    SAS: StateAndStream,
    SAS::State: state::TexState<SAS::State>,
{
    let mut input = ExpansionInputImpl::<SAS> {
        unexpanded_stream: UnexpandedStream::<SAS> {
            stack: vec![],
            state_and_stream,
        },
    };
    loop {
        match input.next() {
            Ok(None) => break,
            Ok(Some(token)) => {
                println!("{:?}", token)
            }
            Err(_) => {
                println!("ERROR");
                break;
            }
        };
    }
}

struct UnexpandedStream<SAS: StateAndStream> {
    stack: Vec<Box<dyn stream::Stream>>,
    state_and_stream: SAS,
}

impl<SAS> stream::Stream for UnexpandedStream<SAS>
where
    SAS: StateAndStream,
    SAS::State: state::TexState<SAS::State>,
{
    fn next(&mut self) -> anyhow::Result<Option<token::Token>> {
        self.prepare_imut_peek()?;
        match self.stack.last_mut() {
            None => self.state_and_stream.stream_mut().next(),
            Some(top) => top.next(),
        }
    }

    fn prepare_imut_peek(&mut self) -> anyhow::Result<()> {
        loop {
            match self.stack.last_mut() {
                None => return self.state_and_stream.stream_mut().prepare_imut_peek(),
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
            None => self.state_and_stream.stream().imut_peek(),
            Some(top) => top.imut_peek(),
        }
    }
}

struct ExpansionInputImpl<SAS: StateAndStream> {
    unexpanded_stream: UnexpandedStream<SAS>,
}

impl<SAS> stream::Stream for ExpansionInputImpl<SAS>
where
    SAS: StateAndStream,
    SAS::State: state::TexState<SAS::State>,
{
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

impl<SAS> primitive::Input<SAS::State> for ExpansionInputImpl<SAS>
where
    SAS: StateAndStream,
    SAS::State: state::TexState<SAS::State>,
{
    fn state(&self) -> &SAS::State {
        self.unexpanded_stream.state_and_stream.state()
    }

    fn state_mut(&mut self) -> &mut SAS::State {
        self.unexpanded_stream.state_and_stream.state_mut()
    }

    fn stream(&mut self) -> &mut dyn Stream {
        self
    }

    fn unexpanded_stream(&mut self) -> &mut dyn Stream {
        &mut self.unexpanded_stream
    }

    fn expand_next(&mut self) -> anyhow::Result<bool> {
        self.unexpanded_stream.prepare_imut_peek()?;
        let command = match self.unexpanded_stream.imut_peek()? {
            None => None,
            Some(token) => match token.value {
                token::Value::Character(..) => None,
                token::Value::ControlSequence(_, ref name) => {
                    self.state().base().primitives.get(name)
                }
            },
        };
        let command = match command {
            Some(primitive::Primitive::Expansion(command)) => command.clone(),
            None => return Ok(false),
        };
        self.unexpanded_stream.consume()?;
        let output = command.call(self)?;
        self.unexpanded_stream.stack.push(output);
        Ok(true)
    }
}
