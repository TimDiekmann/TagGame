use std::collections::HashMap;

/// The agent used in the simulation.
///
/// This defines the behavior on creation, deletion or updates in the simulated environment.
/// Every agent is associated with an `id` and a state. The `id` is unique across one simulation
/// and acts as an identifier. In most cases, it's not needed by the user.
///
/// The state contains data for the agent and can be modified in every function defined by the
/// agent.
///
/// The world is a global state, which can only be changed outside of the simulation and is passed
/// to every agent.
pub trait Agent: Sized + Send + Sync {
    /// The state associated with the agent
    type State: Send + Sync;
    /// The state of the world, provided by the simulation
    type World: Sync;

    /// Called when an agent is added to the simulation.
    #[allow(unused_variables)]
    fn on_creation(&self, id: u64, state: &mut Self::State, world: &Self::World) {}

    /// Called when an agent is removed from the world.
    #[allow(unused_variables)]
    fn on_deletion(&self, id: u64, state: &mut Self::State, world: &Self::World) {}

    /// Called when the simulation is updated.
    #[allow(unused_variables)]
    fn on_update(
        &self,
        id: u64,
        state: &mut Self::State,
        world: &Self::World,
        population: &HashMap<u64, (Self, Self::State)>,
    ) {
    }
}
