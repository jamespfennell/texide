//! Texide primitives API and primitives library.

use crate::tex::state;
use crate::tex::token::stream;

use std::rc;

pub mod library;

use crate::tex::driver;

use crate::tex::state::TexState;
use std::any::TypeId;

pub use driver::ExpandedStream as Input;

// TODO: default clone implementation does not seem to work
#[derive(Copy, Clone)]
pub struct ExpansionStatic<S> {
    call_fn: fn(input: &mut Input<S>) -> anyhow::Result<stream::VecStream>,
    docs: &'static str,
    id: Option<TypeId>,
}

impl<S> ExpansionStatic<S> {
    // TODO: why doesn't clone work
    pub fn duplicate(&self) -> ExpansionStatic<S> {
        ExpansionStatic {
            call_fn: self.call_fn,
            docs: self.docs,
            id: self.id,
        }
    }
}

impl<S: state::TexState<S>> ExpansionPrimitive<S> for ExpansionStatic<S> {
    fn call(&self, input: &mut Input<S>) -> anyhow::Result<stream::VecStream> {
        (self.call_fn)(input)
    }

    fn doc(&self) -> &str {
        self.docs
    }

    fn id(&self) -> Option<TypeId> {
        return self.id;
    }
}

// TODO: rename ExpansionGeneric
pub trait ExpansionPrimitive<S> {
    fn call(&self, input: &mut Input<S>) -> anyhow::Result<stream::VecStream>;

    fn doc(&self) -> &str {
        "this command has no documentation"
    }

    fn id(&self) -> Option<TypeId> {
        None
    }
}

#[derive(Clone)]
pub enum Expansion<S> {
    Static(ExpansionStatic<S>),
    Generic(rc::Rc<dyn ExpansionPrimitive<S>>),
}

impl<S> Expansion<S> {
    pub fn duplicate(&self) -> Expansion<S> {
        match self {
            Expansion::Generic(g) => Expansion::Generic(g.clone()),
            Expansion::Static(s) => Expansion::Static(s.duplicate()),
        }
    }
}

impl<S: TexState<S>> ExpansionPrimitive<S> for Expansion<S> {
    fn call(&self, input: &mut Input<S>) -> anyhow::Result<stream::VecStream> {
        match self {
            Expansion::Static(e) => ExpansionStatic::call(e, input),
            Expansion::Generic(e) => ExpansionPrimitive::call(e.as_ref(), input),
        }
    }

    fn doc(&self) -> &str {
        match self {
            Expansion::Static(e) => ExpansionStatic::doc(e),
            Expansion::Generic(e) => ExpansionPrimitive::doc(e.as_ref()),
        }
    }

    fn id(&self) -> Option<TypeId> {
        match self {
            Expansion::Static(e) => e.id,
            Expansion::Generic(e) => ExpansionPrimitive::id(e.as_ref()),
        }
    }
}

pub enum Primitive<S> {
    Expansion(Expansion<S>),
}

/*
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
*/
