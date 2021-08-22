use std::collections::hash_map::{Entry, HashMap};

use crate::Agent;

/// Adds and removes [`Agent`]s, and updates the them
/// based on their defined behavior.
///
/// Please see the [crate documentation][crate] for examples.
pub struct Simulation<A: Agent> {
    agents: HashMap<u64, (A, A::State)>,
    world: A::World,
    latest_id: u64,
}

impl<A: Agent> Simulation<A> {
    /// Creates a simulation, where different agents can be created.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn new(world: A::World) -> Self {
        Self {
            world,
            agents: HashMap::new(),
            latest_id: 0,
        }
    }

    /// Returns if an agent with the provided `id` is present to the simulation.
    #[inline]
    pub fn has_agent(&self, id: u64) -> bool {
        self.agent(id).is_some()
    }

    /// Returns a reference to the state of the agent identified by the provided id.
    pub fn agent(&self, id: u64) -> Option<&A::State> {
        self.agents.get(&id).map(|(_, s)| s)
    }

    /// Returns a mutable reference to the state of the agent identified by the provided id.
    pub fn agent_mut(&mut self, id: u64) -> Option<&mut A::State> {
        self.agents.get_mut(&id).map(|(_, s)| s)
    }

    /// Returns an iterator over all agents added to the simulation.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn iter(&self) -> impl Iterator<Item = (u64, &A::State)> {
        self.agents.iter().map(|(&id, (_, s))| (id, s))
    }

    /// Returns a mutable iterator over all agents added to the simulation.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u64, &mut A::State)> {
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
    pub fn add_agent(&mut self, agent: A, state: A::State) -> u64 {
        let id = self.latest_id;
        let (agent, state) = if let Entry::Vacant(entry) = self.agents.entry(id) {
            entry.insert((agent, state))
        } else {
            panic!("All {} IDs were used, you beat the system!", u64::MAX)
        };

        agent.on_creation(id, state, &self.world);

        self.latest_id += 1;
        id
    }

    /// Remove an agent by its id.
    ///
    /// Before removing the agent from the simulation, [`Agent::on_deletion`] is called.
    ///
    /// Returns, if the deletion was successful.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn remove_agent(&mut self, id: u64) -> bool {
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
        let agents_copy = self.agents.clone();
        for (&id, (agent, state)) in &mut self.agents {
            agent.on_update(
                id,
                state,
                &self.world,
                agents_copy
                    .iter()
                    .filter(|(&ag_id, _)| ag_id != id)
                    .map(|(&id, (_, s))| (id, s)),
            );
        }
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
    use std::cell::RefCell;

    use crate::{Agent, Simulation};

    struct SimpleAgent;
    impl Agent for SimpleAgent {
        type State = u32;
        type World = ();
    }

    #[test]
    fn test_access() {
        let mut simulation = Simulation::new(());

        let agent_id = simulation.add_agent(SimpleAgent, 42);
        assert!(simulation.has_agent(agent_id));

        assert_eq!(simulation.agent(agent_id), Some(&42));

        *simulation.agent_mut(agent_id).unwrap() = 43;
        assert_eq!(simulation.agent(agent_id), Some(&43));
    }

    #[test]
    fn test_iteration() {
        let mut simulation = Simulation::new(());

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
    struct Counter {
        on_creation_count: u64,
        on_deletion_count: u64,
        on_update_count: u64,
    }

    #[derive(Default)]
    struct CountingAgent {
        counter: RefCell<Counter>,
    }

    impl Agent for &CountingAgent {
        type State = ();
        type World = ();

        fn on_creation(&self, _id: u64, _state: &mut Self::State, _world: &Self::World) {
            self.counter.borrow_mut().on_creation_count += 1;
        }

        fn on_deletion(&self, _id: u64, _state: &mut Self::State, _world: &Self::World) {
            self.counter.borrow_mut().on_deletion_count += 1;
        }

        fn on_update<'sim>(
            &'sim self,
            id: u64,
            _state: &'sim mut Self::State,
            _world: &'sim Self::World,
            mut population: impl Iterator<Item = (u64, &'sim Self::State)>,
        ) {
            self.counter.borrow_mut().on_update_count += 1;
            assert!(!population.any(|(i, _)| i == id));
        }
    }

    #[test]
    fn test_callback() {
        let agent = &CountingAgent::default();

        assert_eq!(agent.counter.borrow().on_creation_count, 0);
        assert_eq!(agent.counter.borrow().on_deletion_count, 0);

        let mut simulation = Simulation::new(());

        let agent_id_1 = simulation.add_agent(agent, ());
        let agent_id_2 = simulation.add_agent(agent, ());
        assert_ne!(agent_id_1, agent_id_2);

        assert_eq!(agent.counter.borrow().on_creation_count, 2);
        assert_eq!(agent.counter.borrow().on_deletion_count, 0);

        assert_eq!(agent.counter.borrow().on_update_count, 0);
        simulation.update();
        assert_eq!(agent.counter.borrow().on_update_count, 2);

        assert!(simulation.remove_agent(agent_id_1));
        assert_eq!(agent.counter.borrow().on_creation_count, 2);
        assert_eq!(agent.counter.borrow().on_deletion_count, 1);

        assert_eq!(agent.counter.borrow().on_update_count, 2);
        simulation.update();
        assert_eq!(agent.counter.borrow().on_update_count, 3);

        assert!(!simulation.remove_agent(agent_id_1));
        assert_eq!(agent.counter.borrow().on_creation_count, 2);
        assert_eq!(agent.counter.borrow().on_deletion_count, 1);

        assert_eq!(agent.counter.borrow().on_update_count, 3);
        simulation.update();
        assert_eq!(agent.counter.borrow().on_update_count, 4);

        drop(simulation);
        assert_eq!(agent.counter.borrow().on_creation_count, 2);
        assert_eq!(agent.counter.borrow().on_deletion_count, 2);
    }
}
