use crate::tex::primitives::expansion;
use std::rc::Rc;

pub trait Expandable<State> {
    fn get_expansion_command(&self, name: &String) -> Option<&Rc<dyn expansion::Command<State>>>;
}
