use crate::datastructures::scopedmap::ScopedMap;
use crate::tex::input;
use crate::tex::primitive;
use crate::tex::primitive::Primitive;
use crate::tex::token::stream;
use std::rc::Rc;

// TeXState is a trait that every state in Texide satisfies. It ensures that the state
// can be used for perform expansion, handle macro processing, etc.
//
// Why parameterized? Because it contains Primitives, which operate on the state
// TODO: do we really need the static constraint here?
pub trait TexState<S> {
    fn base(&self) -> &BaseState<S>;
    fn base_mut(&mut self) -> &mut BaseState<S>;

    /* TODO: this was nice:
    fn input_stream(&self) -> &dyn stream::Stream {
        &self.base().input_module
    }

    fn input_stream_mut(&mut self) -> &dyn stream::Stream {
        &self.base_mut().input_module
    }
    */

    fn get_expansion_primitive(
        &self,
        name: &String,
    ) -> Option<Rc<dyn primitive::ExpansionPrimitive<S>>> {
        if let Some(Primitive::Expansion(p)) = self.base().primitives.get(name) {
            Some(p.clone())
        } else {
            None
        }
    }
}

pub struct BaseState<S> {
    pub primitives: ScopedMap<String, primitive::Primitive<S>>,
    pub input_module: input::InputModule,
}

impl<S> BaseState<S> {
    // Create a new BaseState
    pub fn new() -> BaseState<S> {
        BaseState {
            primitives: ScopedMap::new(),
            input_module: input::InputModule::new(ScopedMap::new()),
        }
    }
}

// SimpleState is the simplest posible state in Texide. It contains only
// the base state. The base state is required for all states.
pub struct SimpleState {
    b: BaseState<SimpleState>,
}

impl SimpleState {
    pub fn new() -> SimpleState {
        SimpleState {
            b: BaseState::new(),
        }
    }
}

impl TexState<SimpleState> for SimpleState {
    fn base(&self) -> &BaseState<SimpleState> {
        &self.b
    }
    fn base_mut(&mut self) -> &mut BaseState<SimpleState> {
        &mut self.b
    }
}
