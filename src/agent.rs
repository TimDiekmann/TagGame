#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An Agent is associated with a unique identifier `id` and
/// its [`State`].
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Agent<S> {
    /// The unique identifier or this agent
    id: u64,
    /// The current state of this agent
    state: S,
}

impl<S> Agent<S> {
    /// Creates a new agent.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let agent = Agent::new(0, "state");
    /// println!("{:?}", agent);
    /// ```
    pub const fn new(id: u64, state: S) -> Self {
        Self { id, state }
    }

    /// Get the agent's identifier.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let agent = Agent::new(0, "state");
    ///
    /// assert_eq!(agent.id(), 0)
    /// ```
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Get a reference to the agent's state.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let agent = Agent::new(0, "state");
    ///
    /// assert_eq!(*agent.state(), "state")
    /// ```
    pub const fn state(&self) -> &S {
        &self.state
    }

    /// Get a mutable reference to the agent's state.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let mut agent = Agent::new(0, "state");
    ///
    /// *agent.state_mut() = "mutated state";
    /// assert_eq!(*agent.state(), "mutated state")
    /// ```
    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    /// Set the agent's state.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let mut agent = Agent::new(0, "state");
    ///
    /// agent.set_state("new state");
    /// assert_eq!(*agent.state(), "new state")
    /// ```
    pub fn set_state(&mut self, state: S) {
        self.state = state;
    }
}
