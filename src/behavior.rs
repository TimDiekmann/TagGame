use crate::{Agent, State};

/// The behavior of an agent.
pub trait Behavior: Sized {
    type State: State;
    type World: State;

    fn on_creation(&self, agent: &Agent<Self::State, Self>, world: &Self::World);
    fn on_deletion(&self, agent: &Agent<Self::State, Self>);
}
