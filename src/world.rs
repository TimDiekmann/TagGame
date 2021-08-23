use crate::Agent;

/// The world holds the global state used in the simulation
///
/// It is updated once in a tick after all [`Agent`] were updated.
/// The [`World`] is able to mutate all states of any agent.
pub trait World<A: Agent> {
    #[allow(unused_variables)]
    /// The update method called when the global states is going to be updated.
    fn update(&mut self, agents: &mut [(A, A::State)]) {}
}

impl<T: Agent> World<T> for () {}
