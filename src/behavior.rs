/// The behavior of an agent.
pub trait Behavior: Sized {
    /// The state associated with the agent
    type State;
    /// The state of the world, provided by the simulation
    type World;

    /// Called when an agent is created to the world.
    #[allow(unused_variables)]
    fn on_creation(&self, id: u64, state: &Self::State, world: &Self::World) {}

    /// Called when an agent is removed from the world.
    #[allow(unused_variables)]
    fn on_deletion(&self, id: u64, state: &Self::State, world: &Self::World) {}

    /// Called when the simulation is updated.
    #[allow(unused_variables)]
    fn on_update<'sim>(
        &'sim self,
        id: u64,
        state: &'sim mut Self::State,
        world: &'sim Self::World,
        population: impl Iterator<Item = (u64, &'sim Self::State)>,
    ) {
    }
}
