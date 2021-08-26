use crate::World;

/// An agent defines, how to act in a simulation.
///
/// The [`Simulation`] associates the agent with a state, which can be mutated
/// by the agent every tick.
///
/// `State` can be seen as data related to the agent itself. The `World` is the global state shared
/// between all agents in a simulation.
///
/// [`Simulation`]: crate::Simulation
pub trait Agent: Sized + Send + Sync {
    /// The local state associated with the agent
    type State: Send + Sync;
    /// The global state, provided by the simulation
    type World: World<Self> + Sync;

    /// Called when an agent is added to the simulation.
    ///
    /// The `id` is a unique id used in the simulation, which is generated when adding the agent to
    /// the simulation. The `state` is the same state passed to [`Simulation::add_agent()`].
    ///
    /// [`Simulation::add_agent()`]: crate::Simulation::add_agent()
    #[allow(unused_variables)]
    fn on_creation(&self, id: usize, state: &mut Self::State, world: &Self::World) {}

    /// Called when the simulation is updated.
    ///
    /// It retrieves the same parameters as [`Agent::on_creation()`] and a list, of all agents,
    /// which are currently added to the simulation. `population[id]` corresponds to this agent.
    #[allow(unused_variables)]
    fn on_update(
        &self,
        id: usize,
        state: &Self::State,
        world: &Self::World,
        population: &[(Self, Self::State)],
    ) -> Option<Self::State>;
}
