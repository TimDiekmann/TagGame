use crate::{Agent, Behavior};

/// Adds and removes [`Agent`]s, and updates the them
/// based on their defined behavior.
///
/// Please see the [crate documentation][crate] for examples.
pub struct Simulation<W, S, B>
where
    B: Behavior<State = S, World = W>,
{
    world: W,
    agents: Vec<Agent<S, B>>,
    latest_id: u64,
}

impl<W, S, B> Simulation<W, S, B>
where
    B: Behavior<State = S, World = W>,
{
    /// Creates a simulation, where different agents can be created.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn new(world: W) -> Self {
        Self {
            world,
            agents: Vec::new(),
            latest_id: 0,
        }
    }

    /// Get a reference to a list of all agents.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn agents(&self) -> &[Agent<S, B>] {
        &self.agents
    }

    /// Add a new agent to the simulation.
    ///
    /// After adding the agent to the simulation, [`Behavior::on_creation`] is called with the
    /// agent and the world state as paramters.
    ///
    /// Returns a unique identifier for the created agent.
    ///
    /// Please see the [crate documentation][crate] for examples.
    pub fn add_agent(&mut self, state: S, behavior: B) -> u64 {
        self.agents
            .push(Agent::new(self.latest_id, state, behavior));

        // `on_creation` has to be called after the agent was added,
        // so the agent is associated to the world
        let agent = self
            .agents
            .last()
            .expect("simulation does not have any agents");
        agent.behavior().on_creation(agent, &self.world);

        self.latest_id += 1;
        agent.id()
    }

    /// Remove an agent by its id.
    ///
    /// Before removing the agent from the simulation, [`Behavior::on_deletion`] is called with the
    /// agent as reference.
    ///
    /// Returns, if the deletion was successful.
    ///
    /// Please see the [crate documentation][crate] for examples.
    // TODO(TimDiekmann): Iterating through an array is O(N), better use a
    //                    slot map or similar here. Will change later if
    //                    enough time left.
    pub fn remove_agent(&mut self, id: u64) -> bool {
        self.agents
            .iter()
            .position(|ag| ag.id() == id)
            .map_or(false, |pos| {
                let agent = &self.agents[pos];
                agent.behavior().on_deletion(agent);

                // Using `Vec::swap_remove` as order doesn't matter for us and deletion is O(1)
                self.agents.swap_remove(pos);
                true
            })
    }
}

impl<W, S, B> Drop for Simulation<W, S, B>
where
    B: Behavior<State = S, World = W>,
{
    fn drop(&mut self) {
        for agent in &self.agents {
            agent.behavior().on_deletion(agent);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{Behavior, Simulation};

    #[derive(Default)]
    struct Counter {
        on_creation_count: u64,
        on_deletion_count: u64,
    }

    #[derive(Default)]
    struct CountingBehavior {
        counter: RefCell<Counter>,
    }

    impl Behavior for &CountingBehavior {
        type State = ();
        type World = ();

        fn on_creation(&self, _agent: &crate::Agent<Self::State, Self>, _world: &Self::World) {
            self.counter.borrow_mut().on_creation_count += 1;
        }

        fn on_deletion(&self, _agent: &crate::Agent<Self::State, Self>) {
            self.counter.borrow_mut().on_deletion_count += 1;
        }
    }

    #[test]
    fn test_add_remove() {
        let behavior = CountingBehavior::default();

        assert_eq!(behavior.counter.borrow().on_creation_count, 0);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 0);

        let mut simulation = Simulation::new(());

        let agent_id_1 = simulation.add_agent((), &behavior);
        let agent_id_2 = simulation.add_agent((), &behavior);
        assert_ne!(agent_id_1, agent_id_2);
        assert_eq!(behavior.counter.borrow().on_creation_count, 2);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 0);

        assert!(simulation.remove_agent(agent_id_1));
        assert_eq!(behavior.counter.borrow().on_creation_count, 2);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 1);

        assert!(!simulation.remove_agent(agent_id_1));
        assert_eq!(behavior.counter.borrow().on_creation_count, 2);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 1);

        drop(simulation);
        assert_eq!(behavior.counter.borrow().on_creation_count, 2);
        assert_eq!(behavior.counter.borrow().on_deletion_count, 2);
    }
}
