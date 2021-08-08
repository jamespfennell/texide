//! Texide primitives API and primitives library.

use crate::datastructures::scopedmap::ScopedMap;
use crate::tex::driver;
use crate::tex::driver::StateAndStream;
use crate::tex::state::{BaseState, TexState};
use crate::tex::token::catcode::CatCode;
use crate::tex::token::stream;
use crate::tex::token::token;
use library::{conditional, texide};
use std::rc;

pub mod library;

use std::any::{Any, TypeId};

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

// TODO: rename expansion
// TODO: clonable?
pub trait ExpansionPrimitive<State>: Any {
    fn call(&self, input: &mut dyn Input<State>) -> anyhow::Result<Box<dyn stream::Stream>>;
    // TODO: add docs

    fn id(&self) -> Option<TypeId> {
        None
    }
}

pub enum Primitive<State> {
    Expansion(rc::Rc<dyn ExpansionPrimitive<State>>),
}

// TEST STATE
struct TestState {
    base_state: BaseState<TestState>,
}

impl TexState<TestState> for TestState {
    fn base(&self) -> &BaseState<TestState> {
        &self.base_state
    }

    fn base_mut(&mut self) -> &mut BaseState<TestState> {
        &mut self.base_state
    }
}

struct TestStateWithStream {
    st: TestState,
    sr: stream::VecStream,
}

// TODO: when will it never be like this? When the state is contained in the stream?
impl driver::StateAndStream for TestStateWithStream {
    type State = TestState;

    fn state(&self) -> &TestState {
        &self.st
    }

    fn state_mut(&mut self) -> &mut TestState {
        &mut self.st
    }

    fn stream(&self) -> &dyn stream::Stream {
        &self.sr
    }

    fn stream_mut(&mut self) -> &mut dyn stream::Stream {
        &mut self.sr
    }
}

// END TEST STATE

/*
struct StaticCommand<State: 'static> {
    command: fn(input: &mut dyn expansion::Input<State>) -> anyhow::Result<expansion::Output>,
}

impl<State> expansion::Command<State> for StaticCommand<State> {
    fn run(&self, input: &mut dyn expansion::Input<State>) -> anyhow::Result<expansion::Output> {
        (self.command)(input)
    }
}*/

pub fn expand() {
    let mut commands: ScopedMap<String, Primitive<TestState>> = ScopedMap::new();
    commands.insert(
        "james".to_string(),
        Primitive::Expansion(rc::Rc::new(texide::get_texide())),
    );
    commands.insert(
        "if".to_string(),
        Primitive::Expansion(rc::Rc::new(conditional::get_if())),
    );
    commands.insert(
        "else".to_string(),
        Primitive::Expansion(rc::Rc::new(conditional::get_else())),
    );
    commands.insert(
        "fi".to_string(),
        Primitive::Expansion(rc::Rc::new(conditional::get_fi())),
    );
    /*commands.insert(
        "james2".to_string(),
        rc::Rc::new(StaticCommand {
            command: texide::texide_command,
        }),
    );

     */

    //let mut commands2: ScopedMap<String, fn(&mut TestState) -> anyhow::Result<expansion::Output>> = ScopedMap::new();
    //commands2.insert("hey".to_string(), texide::texide_command);

    //let a : Box<dyn GetExpansionCommand<TestState>> = Box::new(state);
    let i_stream = stream::VecStream::new(vec![
        create_token('i'),
        create_token('n'),
        create_token('p'),
        create_cmd_token("mint".to_string()),
        create_cmd_token("if".to_string()),
        create_cmd_token("james".to_string()),
        create_token('u'),
        create_cmd_token("else".to_string()),
        create_token('u'),
        create_token('t'),
    ]);
    let mut state_backed_stream = TestStateWithStream {
        st: TestState {
            base_state: BaseState::new(),
        },
        sr: i_stream,
    };
    let b = state_backed_stream.state_mut().base_mut();
    b.primitives = commands;
    driver::run(state_backed_stream);
}

fn create_token(c: char) -> token::Token {
    token::Token {
        value: token::Value::Character(c, CatCode::Letter),
        // TODO: should not have to specify source or something
        // TODO: clearly we need to constructors for tokens
        // TODO: maybe even a nice constructor for VecStream?
        source: None,
    }
}
fn create_cmd_token(s: String) -> token::Token {
    token::Token {
        value: token::Value::ControlSequence('\\', s),
        source: None,
    }
}
