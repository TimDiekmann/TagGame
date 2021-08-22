use rayon::prelude::*;

use crate::{Agent, World};

/// Adds and removes [`Agent`]s, and updates the them
/// based on their defined behavior.
///
/// Please see the [crate documentation][crate] for examples.
pub struct Simulation<A: Agent> {
    agents: Vec<(A, A::State)>,
    state_buffer: Vec<(A, A::State)>,
    world: A::World,
}

impl<A: Agent> Simulation<A> {
    /// Creates a simulation, where different agents can be created.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn new(world: A::World) -> Self {
        Self {
            world,
            agents: Vec::new(),
            state_buffer: Vec::new(),
        }
    }

    /// Returns a slice over all agents added to the simulation.
    ///
    /// Please see the [crate documentation][crate] for examples.
    #[inline]
    pub fn agents(&self) -> &[(A, A::State)] {
        &self.agents
    }

    /// Returns a mutable iterator over all agents added to the simulation.
    ///
    /// Please see the [crate documentation][crate] for examples.
    #[inline]
    pub fn agents_mut(&mut self) -> &mut [(A, A::State)] {
        &mut self.agents
    }

    /// Add a new agent to the simulation.
    ///
    /// After adding the agent to the simulation, [`Agent::on_creation`] is called.
    ///
    /// Returns a unique identifier for the created agent.
    ///
    /// Please see the [crate documentation][crate] for examples.
    ///
    /// # Panics
    ///
    /// When the simulation runs out of unique identifiers (2^64).
    pub fn add_agent(&mut self, agent: A, mut state: A::State) -> usize {
        let id = self.agents.len();
        agent.on_creation(id, &mut state, &self.world);
        self.agents.push((agent, state));
        id
    }

    /// Get a reference to the simulation's world.
    pub fn world(&self) -> &A::World {
        &self.world
    }

    /// Get a mutable reference to the simulation's world.
    pub fn world_mut(&mut self) -> &mut A::World {
        &mut self.world
    }
}

impl<A: Agent> Simulation<A>
where
    A: Clone,
    A::State: Clone,
{
    /// Calls [`Agent::on_update`] for every registered agent.
    pub fn update(&mut self) {
        self.state_buffer.clone_from(&self.agents);
        let state_buffer = &self.state_buffer;
        let world = &self.world;
        self.agents
            .par_iter_mut()
            .enumerate()
            .for_each(|(id, (agent, state))| {
                agent.on_update(id, state, world, state_buffer);
            });
        self.world.update(&mut self.agents);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::atomic::{AtomicU64, Ordering},
    };

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
