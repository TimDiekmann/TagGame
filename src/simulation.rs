use std::collections::hash_map::{Entry, HashMap};

use crate::Agent;

/// Adds and removes [`Agent`]s, and updates the them
/// based on their defined behavior.
///
/// Please see the [crate documentation][crate] for examples.
pub struct Simulation<W, S, B>
where
    B: Agent<State = S, World = W>,
{
    world: W,
    agents: HashMap<u64, (S, B)>,
    latest_id: u64,
}

impl<W, S, B> Simulation<W, S, B>
where
    B: Agent<State = S, World = W>,
{
    /// Creates a simulation, where different agents can be created.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn new(world: W) -> Self {
        Self {
            world,
            agents: HashMap::new(),
            latest_id: 0,
        }
    }

    /// Returns if an agent with the provided `id` is added to the simulation.
    #[inline]
    pub fn has_agent(&self, id: u64) -> bool {
        self.state(id).is_some()
    }

    /// Returns a reference to the state of the agent identified by the provided id.
    pub fn state(&self, id: u64) -> Option<&S> {
        self.agents.get(&id).map(|(s, _)| s)
    }

    /// Returns a mutable reference to the state of the agent identified by the provided id.
    pub fn state_mut(&mut self, id: u64) -> Option<&mut S> {
        self.agents.get_mut(&id).map(|(s, _)| s)
    }

    /// Returns a reference to the behavior of the agent identified by the provided id.
    pub fn behavior(&self, id: u64) -> Option<&B> {
        self.agents.get(&id).map(|(_, b)| b)
    }

    /// Returns a mutable reference to the behavior of the agent identified by the provided id.
    pub fn behavior_mut(&mut self, id: u64) -> Option<&mut B> {
        self.agents.get_mut(&id).map(|(_, b)| b)
    }

    /// Returns an iterator over all agents added to the simulation.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn agents(&self) -> impl Iterator<Item = (u64, &S)> {
        self.agents.iter().map(|ag| (*ag.0, &(ag.1).0))
    }

    /// Returns a mutable iterator over all agents added to the simulation.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn agents_mut(&mut self) -> impl Iterator<Item = (u64, &mut S)> {
        self.agents.iter_mut().map(|ag| (*ag.0, &mut (ag.1).0))
    }

    /// Add a new agent to the simulation.
    ///
    /// After adding the agent to the simulation, [`Behavior::on_creation`] is called with the
    /// agent and the world state as paramters.
    ///
    /// Returns a unique identifier for the created agent.
    ///
    /// Please see the [crate documentation][crate] for examples.
    ///
    /// # Panics
    ///
    /// When the simulation runs out of IDs.
    pub fn add_agent(&mut self, state: S, behavior: B) -> u64 {
        let id = self.latest_id;
        let (state, behavior) = if let Entry::Vacant(entry) = self.agents.entry(id) {
            entry.insert((state, behavior))
        } else {
            panic!("All {} IDs were used, you beat the system!", u64::MAX)
        };

        behavior.on_creation(id, state, &self.world);

        self.latest_id += 1;
        id
    }

    /// Remove an agent by its id.
    ///
    /// Before removing the agent from the simulation, [`Behavior::on_deletion`] is called with the
    /// agent as reference.
    ///
    /// Returns, if the deletion was successful.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn remove_agent(&mut self, id: u64) -> bool {
        if let Entry::Occupied(entry) = self.agents.entry(id) {
            let (state, behavior) = entry.get();
            behavior.on_deletion(id, state, &self.world);
            entry.remove();
            true
        } else {
            false
        }
    }
}

impl<W, S, B> Simulation<W, S, B>
where
    S: Clone,
    B: Agent<State = S, World = W> + Clone,
{
    /// Calls [`Behavior::on_update`] for every registered agent.
    ///
    /// Every agent has mutable access to its own state and immutable access to its id, the world,
    /// and all other agents.
    pub fn update(&mut self) {
        let agents_copy = self.agents.clone();
        for (&id, (state, behavior)) in &mut self.agents {
            behavior.on_update(
                id,
                state,
                &self.world,
                agents_copy
                    .iter()
                    .filter(|(&ag_id, _)| ag_id != id)
                    .map(|(&id, (s, _))| (id, s)),
            );
        }
    }
}

impl<W, S, B> Drop for Simulation<W, S, B>
where
    B: Agent<State = S, World = W>,
{
    fn drop(&mut self) {
        for (&id, (state, behavior)) in &self.agents {
            behavior.on_deletion(id, state, &self.world);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{Agent, Simulation};

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

        fn on_creation(&self, _id: u64, _state: &Self::State, _world: &Self::World) {
            self.counter.borrow_mut().on_creation_count += 1;
        }

        fn on_deletion(&self, _id: u64, _state: &Self::State, _world: &Self::World) {
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
    fn test_add_remove() {
        let behavior = CountingAgent::default();

        assert_eq!(behavior.counter.borrow().on_creation_count, 0);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 0);

        let mut simulation = Simulation::new(());

        let agent_id_1 = simulation.add_agent((), &behavior);
        let agent_id_2 = simulation.add_agent((), &behavior);
        assert_ne!(agent_id_1, agent_id_2);
        assert_eq!(behavior.counter.borrow().on_creation_count, 2);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 0);

        assert_eq!(behavior.counter.borrow().on_update_count, 0);
        simulation.update();
        assert_eq!(behavior.counter.borrow().on_update_count, 2);

        assert!(simulation.remove_agent(agent_id_1));
        assert_eq!(behavior.counter.borrow().on_creation_count, 2);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 1);

        assert_eq!(behavior.counter.borrow().on_update_count, 2);
        simulation.update();
        assert_eq!(behavior.counter.borrow().on_update_count, 3);

        assert!(!simulation.remove_agent(agent_id_1));
        assert_eq!(behavior.counter.borrow().on_creation_count, 2);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 1);

        assert_eq!(behavior.counter.borrow().on_update_count, 3);
        simulation.update();
        assert_eq!(behavior.counter.borrow().on_update_count, 4);

        drop(simulation);
        assert_eq!(behavior.counter.borrow().on_creation_count, 2);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 2);
    }
}
