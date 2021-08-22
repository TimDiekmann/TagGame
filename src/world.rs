use std::collections::HashMap;

use crate::{Agent, Id};

pub trait World<A: Agent> {
    #[allow(unused_variables)]
    fn update(&mut self, agents: &mut HashMap<Id, (A, A::State)>) {}
}

impl<T: Agent> World<T> for () {}
