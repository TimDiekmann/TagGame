use crate::Agent;

pub trait World<A: Agent> {
    #[allow(unused_variables)]
    fn update(&mut self, agents: &mut [(A, A::State)]) {}
}

impl<T: Agent> World<T> for () {}
