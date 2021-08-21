use crate::Agent;

/// The behavior of an agent.
pub trait Behavior: Sized {
    /// The state associated with the agent
    type State;
    /// The state of the world, provided by the simulation
    type World;

    /// Called when an agent is created to the world.
    fn on_creation(&self, agent: &Agent<Self::State, Self>, world: &Self::World) {
        let _ = agent;
        let _ = world;
    }

    /// Called when an agent is removed from the world.
    fn on_deletion(&self, agent: &Agent<Self::State, Self>) {
        let _ = agent;
    }

    /// Called when the simulation is updated.
    fn on_update<'sim>(
        &'sim self,
        id: u64,
        state: &'sim mut Self::State,
        world: &'sim Self::World,
        population: impl Iterator<Item = Agent<'sim, Self::State, Self>>,
    ) {
        let _ = id;
        let _ = state;
        let _ = world;
        drop(population);
    }
}
