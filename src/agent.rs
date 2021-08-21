/// An Agent is associated with a unique identifier `id` and
/// its state. It should not be created directly but with
/// [`Simulation::add_agent()`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Agent<'sim, S, B> {
    /// The unique identifier or this agent.
    id: u64,
    /// The current state of this agent.
    state: &'sim S,
    /// The behavior or this agent.
    behavior: &'sim B,
}

impl<'sim, S, B> Agent<'sim, S, B> {
    /// Creates a new agent with the provided identifier, the state, and the behavior.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let agent = Agent::new(0, &"state", &());
    /// println!("{:?}", agent);
    /// ```
    pub const fn new(id: u64, state: &'sim S, behavior: &'sim B) -> Self {
        Self {
            id,
            state,
            behavior,
        }
    }

    /// Get the agent's identifier.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let agent = Agent::new(0, &"state", &());
    ///
    /// assert_eq!(agent.id(), 0);
    /// ```
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Get a reference to the agent's state.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let agent = Agent::new(0, &"state", &());
    ///
    /// assert_eq!(*agent.state(), "state");
    /// ```
    pub const fn state(&self) -> &S {
        self.state
    }

    /// Get a reference to the agent's behavior.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let mut agent = Agent::new(0, &"state", &10);
    ///
    /// assert_eq!(*agent.behavior(), 10);
    /// ```
    pub const fn behavior(&self) -> &B {
        self.behavior
    }
}

#[cfg(test)]
mod tests {
    use crate::Agent;

    #[test]
    fn test_agent() {
        let agent = Agent::new(0, &"0", &0);
        assert_eq!(agent.id(), 0);

        assert_eq!(*agent.state(), "0");
        assert_eq!(*agent.behavior(), 0);
    }
}
