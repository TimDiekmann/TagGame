use std::collections::HashMap;

use crate::Agent;

pub trait World<A: Agent> {
    #[allow(unused_variables)]
    fn update(&mut self, agents: &mut HashMap<u64, (A, A::State)>) {}
}

impl<T: Agent> World<T> for () {}
