use crate::datastructures::scopedmap::ScopedMap;
use crate::tex::primitive;
use crate::tex::primitive::Primitive;
use std::rc::Rc;

// TeXState is a trait that every state in Texide satisfies. It ensures that the state
// can be used for perform expansion, handle macro processing, etc.
//
// Why parameterized? Because it contains Primitives, which operate on the state
pub trait TexState<S> {
    fn base(&self) -> &BaseState<S>;
    fn base_mut(&mut self) -> &mut BaseState<S>;

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
}

impl<S> BaseState<S> {
    // Create a new BaseState
    pub fn new() -> BaseState<S> {
        BaseState {
            primitives: ScopedMap::new(),
        }
    }
}
