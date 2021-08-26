use rayon::prelude::*;

use crate::{agent, Agent, World};

/// Keeps track of all [`Agent`]s, its states and the global state.
///
/// It's responsible to update the `Agent`s states based on their defined behavior.
///
/// Please see the [crate documentation][crate] for examples.
pub struct Simulation<A: Agent> {
    agents: Vec<(A, A::State)>,
    state_buffer: Vec<(A, A::State)>,
    world: A::World,
}

impl<A: Agent> Simulation<A> {
    /// Creates a simulation with the provided [`World`].
    ///
    /// The `World` is passed to the agents, when they get update.
    pub fn new(world: A::World) -> Self {
        Self {
            world,
            agents: Vec::new(),
            state_buffer: Vec::new(),
        }
    }

    /// Creates a simulation with the provided [`World`] and reserves memory for the spcified
    /// number of agent without the need for reallocating.
    pub fn with_capacity(world: A::World, num_agent: usize) -> Self {
        Self {
            world,
            agents: Vec::with_capacity(num_agent),
            state_buffer: Vec::with_capacity(num_agent),
        }
    }

    /// Returns a slice over all agents and its state added to the simulation.
    #[inline]
    pub fn agents(&self) -> &[(A, A::State)] {
        &self.agents
    }

    /// Returns a mutable slice over all agents and its state added to the simulation.
    #[inline]
    pub fn agents_mut(&mut self) -> &mut [(A, A::State)] {
        &mut self.agents
    }

    /// Add a new agent with an initial to the simulation.
    ///
    /// This will call [`Agent::on_creation()`] when the agent was created.
    ///
    /// Returns a unique identifier for the created agent.
    pub fn add_agent(&mut self, agent: A, mut state: A::State) -> usize {
        let id = self.agents.len();
        agent.on_creation(id, &mut state, &self.world);
        self.agents.push((agent, state));
        id
    }

    /// Get a shared reference to the global state.
    pub fn world(&self) -> &A::World {
        &self.world
    }

    /// Get a unique reference to the global state.
    pub fn world_mut(&mut self) -> &mut A::World {
        &mut self.world
    }
}

impl<A: Agent> Simulation<A>
where
    A: Clone,
    A::State: Clone,
{
    /// Advances the simulation by one tick.
    ///
    /// First, on every added Agent, [`Agent::on_update()`] is invoked in arbitrary order.
    /// This happens in parallel. Afterwards, the global state is updated by calling
    /// [`World::update()`].
    ///
    /// To every [`Agent`] it's current state is passed as unique reference. Also a list
    /// of all other agents is passed as shared reference. The list is updated once before
    /// the tick and every agent will receive the same list.
    ///
    /// When updating the global state, a mutable slice to all `Agent`s and its states
    /// are passed to [`World`].
    pub fn update(&mut self) {
        let new_states: Vec<(usize, A::State)> = self
            .agents
            .par_iter()
            .enumerate()
            .filter_map(|(id, (agent, state))| {
                agent
                    .on_update(id, state, &self.world, &self.agents)
                    .map(|x| (id, x))
            })
            .collect();
        for (i, updated_state) in new_states {
            self.agents[i].1 = updated_state;
        }
        self.world.update(&mut self.agents);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use crate::{Agent, Simulation, World};

    #[derive(Debug, PartialEq, Eq)]
    struct SimpleWorld(&'static str);

    struct SimpleAgent;
    impl World<SimpleAgent> for SimpleWorld {}
    impl Agent for SimpleAgent {
        type State = usize;
        type World = SimpleWorld;
    }

    #[test]
    fn test_world() {
        let mut simulation = Simulation::<SimpleAgent>::new(SimpleWorld("world"));

        assert_eq!(*simulation.world(), SimpleWorld("world"));

        simulation.world_mut().0 = "hello";
        assert_eq!(*simulation.world(), SimpleWorld("hello"));
    }

    #[test]
    fn test_iteration() {
        let mut simulation = Simulation::new(SimpleWorld("world"));

        let agent_ids = (0..4)
            .map(|i| simulation.add_agent(SimpleAgent, i))
            .collect::<Vec<_>>();

        assert_eq!(simulation.agents().len(), 4);

        for id in &agent_ids {
            assert!(simulation.agents().iter().any(|(_, i)| i == id));
        }
        simulation
            .agents_mut()
            .iter_mut()
            .for_each(|(_, s)| *s *= *s);
        for state in 0..4 {
            assert!(simulation.agents().iter().any(|(_, s)| *s == state * state));
        }
    }

    #[derive(Default)]
    struct CountingAgent {
        on_creation_count: AtomicU64,
        on_update_count: AtomicU64,
    }
    impl Agent for &CountingAgent {
        type State = ();
        type World = ();

        fn on_creation(&self, _id: usize, _state: &mut Self::State, _world: &Self::World) {
            self.on_creation_count.fetch_add(1, Ordering::Relaxed);
        }

        fn on_update<'sim>(
            &'sim self,
            _id: usize,
            _state: &'sim mut Self::State,
            _world: &'sim Self::World,
            _population: &[(Self, Self::State)],
        ) {
            self.on_update_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_update() {
        let agent = &CountingAgent::default();
        let mut simulation = Simulation::new(());
        simulation.add_agent(agent, ());

        assert_eq!(agent.on_update_count.load(Ordering::SeqCst), 0);
        simulation.update();
        assert_eq!(agent.on_update_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_callback() {
        let agent = &CountingAgent::default();

        assert_eq!(agent.on_creation_count.load(Ordering::Relaxed), 0);

        let mut simulation = Simulation::new(());

        let agent_id_1 = simulation.add_agent(agent, ());
        let agent_id_2 = simulation.add_agent(agent, ());
        assert_ne!(agent_id_1, agent_id_2);

        assert_eq!(agent.on_creation_count.load(Ordering::Relaxed), 2);
    }
}
