use std::collections::hash_map::{Entry, HashMap};

use rayon::prelude::*;

use crate::{Agent, World};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Id(pub u64);

/// Adds and removes [`Agent`]s, and updates the them
/// based on their defined behavior.
///
/// Please see the [crate documentation][crate] for examples.
pub struct Simulation<A: Agent> {
    agents: HashMap<Id, (A, A::State)>,
    state_buffer: HashMap<Id, (A, A::State)>,
    world: A::World,
    latest_id: Id,
}

impl<A: Agent> Simulation<A> {
    /// Creates a simulation, where different agents can be created.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn new(world: A::World) -> Self {
        Self {
            world,
            agents: HashMap::new(),
            latest_id: Id(0),
            state_buffer: HashMap::new(),
        }
    }

    /// Returns if an agent with the provided `id` is present to the simulation.
    #[inline]
    pub fn has_agent(&self, id: Id) -> bool {
        self.agent(id).is_some()
    }

    /// Returns a reference to the state of the agent identified by the provided id.
    pub fn agent(&self, id: Id) -> Option<&A::State> {
        self.agents.get(&id).map(|(_, s)| s)
    }

    /// Returns a mutable reference to the state of the agent identified by the provided id.
    pub fn agent_mut(&mut self, id: Id) -> Option<&mut A::State> {
        self.agents.get_mut(&id).map(|(_, s)| s)
    }

    /// Returns an iterator over all agents added to the simulation.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn iter(&self) -> impl Iterator<Item = (Id, &A::State)> {
        self.agents.iter().map(|(&id, (_, s))| (id, s))
    }

    /// Returns a mutable iterator over all agents added to the simulation.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut A::State)> {
        self.agents.iter_mut().map(|(&id, (_, s))| (id, s))
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
    pub fn add_agent(&mut self, agent: A, state: A::State) -> Id {
        let id = self.latest_id;
        let (agent, state) = if let Entry::Vacant(entry) = self.agents.entry(id) {
            entry.insert((agent, state))
        } else {
            panic!("All {} IDs were used, you beat the system!", u64::MAX)
        };

        agent.on_creation(id, state, &self.world);

        self.latest_id.0 += 1;
        id
    }

    /// Remove an agent by its id.
    ///
    /// Before removing the agent from the simulation, [`Agent::on_deletion`] is called.
    ///
    /// Returns, if the deletion was successful.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn remove_agent(&mut self, id: Id) -> bool {
        if let Entry::Occupied(mut entry) = self.agents.entry(id) {
            let (agent, state) = entry.get_mut();
            agent.on_deletion(id, state, &self.world);
            entry.remove();
            true
        } else {
            false
        }
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
            .for_each(|(&id, (agent, state))| {
                agent.on_update(id, state, world, state_buffer);
            });
        self.world.update(&mut self.agents);
    }
}

impl<A: Agent> Drop for Simulation<A> {
    fn drop(&mut self) {
        for (&id, (agent, state)) in &mut self.agents {
            agent.on_deletion(id, state, &self.world);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::atomic::{AtomicU64, Ordering},
    };

    use crate::{Agent, Id, Simulation, World};

    #[derive(Debug, PartialEq, Eq)]
    struct SimpleWorld(&'static str);

    struct SimpleAgent;
    impl World<SimpleAgent> for SimpleWorld {}
    impl Agent for SimpleAgent {
        type State = u32;
        type World = SimpleWorld;
    }

    #[test]
    fn test_access() {
        let mut simulation = Simulation::new(SimpleWorld("world"));

        let agent_id = simulation.add_agent(SimpleAgent, 42);
        assert!(simulation.has_agent(agent_id));

        assert_eq!(simulation.agent(agent_id), Some(&42));

        *simulation.agent_mut(agent_id).unwrap() = 43;
        assert_eq!(simulation.agent(agent_id), Some(&43));
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

        let agent_ids = (1..=4)
            .map(|i| simulation.add_agent(SimpleAgent, i))
            .collect::<Vec<_>>();

        assert_eq!(simulation.iter().count(), 4);

        for &id in &agent_ids {
            assert!(simulation.iter().any(|(i, _)| i == id));
        }
        simulation.iter_mut().for_each(|(_, s)| *s *= *s);
        for state in 1..=4 {
            assert!(simulation.iter().any(|(_, &s)| s == state * state));
        }
    }

    #[derive(Default)]
    struct CountingAgent {
        on_creation_count: AtomicU64,
        on_deletion_count: AtomicU64,
        on_update_count: AtomicU64,
    }
    impl Agent for &CountingAgent {
        type State = ();
        type World = ();

        fn on_creation(&self, _id: Id, _state: &mut Self::State, _world: &Self::World) {
            self.on_creation_count.fetch_add(1, Ordering::Relaxed);
        }

        fn on_deletion(&self, _id: Id, _state: &mut Self::State, _world: &Self::World) {
            self.on_deletion_count.fetch_add(1, Ordering::Relaxed);
        }

        fn on_update<'sim>(
            &'sim self,
            _id: Id,
            _state: &'sim mut Self::State,
            _world: &'sim Self::World,
            _population: &HashMap<Id, (Self, Self::State)>,
        ) {
            self.on_update_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_update() {
        let agent = &CountingAgent::default();
        let mut simulation = Simulation::new(());
        let agent_id = simulation.add_agent(agent, ());

        assert_eq!(agent.on_update_count.load(Ordering::SeqCst), 0);
        simulation.update();
        assert_eq!(agent.on_update_count.load(Ordering::SeqCst), 1);

        assert!(simulation.remove_agent(agent_id));

        assert_eq!(agent.on_update_count.load(Ordering::SeqCst), 1);
        simulation.update();
        assert_eq!(agent.on_update_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_callback() {
        let agent = &CountingAgent::default();

        assert_eq!(agent.on_creation_count.load(Ordering::Relaxed), 0);
        assert_eq!(agent.on_deletion_count.load(Ordering::Relaxed), 0);

        let mut simulation = Simulation::new(());

        let agent_id_1 = simulation.add_agent(agent, ());
        let agent_id_2 = simulation.add_agent(agent, ());
        assert_ne!(agent_id_1, agent_id_2);

        assert_eq!(agent.on_creation_count.load(Ordering::Relaxed), 2);
        assert_eq!(agent.on_deletion_count.load(Ordering::Relaxed), 0);

        assert!(simulation.remove_agent(agent_id_1));
        assert_eq!(agent.on_creation_count.load(Ordering::Relaxed), 2);
        assert_eq!(agent.on_deletion_count.load(Ordering::Relaxed), 1);

        assert!(!simulation.remove_agent(agent_id_1));
        assert_eq!(agent.on_creation_count.load(Ordering::Relaxed), 2);
        assert_eq!(agent.on_deletion_count.load(Ordering::Relaxed), 1);

        drop(simulation);
        assert_eq!(agent.on_creation_count.load(Ordering::Relaxed), 2);
        assert_eq!(agent.on_deletion_count.load(Ordering::Relaxed), 2);
    }
}
