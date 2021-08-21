use crate::Agent;

/// The behavior of an agent.
pub trait Behavior: Sized {
    /// The state associated with the agent
    type State;
    /// The state of the world, provided by the simulation
    type World;

    /// Called when an agent is created to the world.
    fn on_creation(&self, agent: &Agent<Self::State, Self>, world: &Self::World);

    /// Called when an agent is removed from the world.
    fn on_deletion(&self, agent: &Agent<Self::State, Self>);
}
