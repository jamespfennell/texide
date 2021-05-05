//! Texide primitives API and primitives library.

use crate::datastructures::scopedmap::ScopedMap;
use crate::tex::token::catcode::CatCode;
use crate::tex::token::stream;
use crate::tex::token::token;
use crate::tex::{driver, state};
use library::texide;
use std::rc::Rc;

pub mod library;

/// Texide API for expansion primitives.
pub mod expansion {
    use crate::tex::token::stream;

    // This is the trait that will be passed into
    pub enum Output {
        None,
        Vec(stream::VecStream),
        Other(Box<dyn stream::Stream>),
    }

    pub trait Input<State> {
        /// Returns an immutable reference to the underlying state.
        fn state(&self) -> &State;

        /// Returns a mutable reference to the underlying state.
        fn state_mut(&mut self) -> &mut State;

        /// Returns a stream that is the input stream but with expansion performed.
        fn stream(&mut self) -> &mut dyn stream::Stream;

        /// Returns the unexpanded input stream.
        fn unexpanded_stream(&mut self) -> &mut dyn stream::Stream;

        /// Expand the next token in the input stream, if it is an expansion command.
        /// Returns true iff expansion occurred.
        fn expand_next(&mut self) -> anyhow::Result<bool>;
    }

    pub trait Command<State> {
        fn run(&self, input: &mut dyn Input<State>) -> anyhow::Result<Output>;
        // TODO: add docs
    }
}

/// Texide API for execution primitives.
pub mod execution {}

// TEST STATE
struct TestState {
    commands: ScopedMap<String, Rc<dyn expansion::Command<TestState>>>,
}

impl state::Expandable<TestState> for TestState {
    fn get_expansion_command(
        &self,
        name: &String,
    ) -> Option<&Rc<dyn expansion::Command<TestState>>> {
        return self.commands.get(name);
    }
}

struct TestStateWithStream {
    state: TestState,
    stream: stream::VecStream,
}

impl driver::StateAndStream for TestStateWithStream {
    type State = TestState;

    fn state(&self) -> &TestState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut TestState {
        &mut self.state
    }

    fn stream(&self) -> &dyn stream::Stream {
        &self.stream
    }

    fn stream_mut(&mut self) -> &mut dyn stream::Stream {
        &mut self.stream
    }
}

// END TEST STATE

pub fn expand() {
    let mut commands: ScopedMap<String, Rc<dyn expansion::Command<TestState>>> = ScopedMap::new();
    commands.insert("james".to_string(), Rc::new(texide::get_texide()));

    //let a : Box<dyn GetExpansionCommand<TestState>> = Box::new(state);
    let i_stream = stream::VecStream::new(vec![
        create_token('i'),
        create_token('n'),
        create_token('p'),
        create_cmd_token("mint".to_string()),
        create_cmd_token("james".to_string()),
        create_token('u'),
        create_token('t'),
    ]);
    let state_backed_stream = TestStateWithStream {
        state: TestState { commands },
        stream: i_stream,
    };

    driver::run(state_backed_stream);
}

fn create_token(c: char) -> token::Token {
    token::Token {
        value: token::Value::Character(c, CatCode::Letter),
        // TODO: should not have to specify source or something
        // TODO: clearly we need to constructors for tokens
        // TODO: maybe even a nice constructor for VecStream?
        source: token::Source {
            line: Rc::new(token::Line {
                content: "".to_string(),
                line_number: 0,
                file: Rc::new("".to_string()),
            }),
            position: 0,
        },
    }
}
fn create_cmd_token(s: String) -> token::Token {
    token::Token {
        value: token::Value::ControlSequence('\\', s),
        source: token::Source {
            line: Rc::new(token::Line {
                content: "".to_string(),
                line_number: 0,
                file: Rc::new("".to_string()),
            }),
            position: 0,
        },
    }
}
