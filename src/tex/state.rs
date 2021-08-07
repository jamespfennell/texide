use crate::tex::primitive::expansion;
use std::rc::Rc;
use crate::datastructures::scopedmap::ScopedMap;

// TeXState is a trait that every state in Texide satisfies. It ensures that the state
// can be used for perform expansion, handle macro processing, etc.
pub trait TexState<S> {
    fn base(&self) -> &BaseState<S>;
    fn base_mut(&mut self) -> &mut BaseState<S>;
}

pub struct BaseState<S> {
    pub primitives: ScopedMap<String, Rc<dyn expansion::Command<S>>>
}

impl<S> BaseState<S> {
    // Create a new BaseState
    pub fn new() -> BaseState<S> {
        BaseState {
            primitives: ScopedMap::new()
        }
    }
}
