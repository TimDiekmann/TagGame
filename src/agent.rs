#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An Agent is associated with a unique identifier `id` and
/// its state.
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Agent<S, B> {
    /// The unique identifier or this agent.
    id: u64,
    /// The current state of this agent.
    state: S,
    /// The behavior or this agent.
    behavior: B,
}

impl<S, B> Agent<S, B> {
    /// Creates a new agent with the provided identifier, the state, and the behavior.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let agent = Agent::new(0, "state", ());
    /// println!("{:?}", agent);
    /// ```
    pub const fn new(id: u64, state: S, behavior: B) -> Self {
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
    /// let agent = Agent::new(0, "state", ());
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
    /// let agent = Agent::new(0, "state", ());
    ///
    /// assert_eq!(*agent.state(), "state");
    /// ```
    pub const fn state(&self) -> &S {
        &self.state
    }

    /// Get a mutable reference to the agent's state.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let mut agent = Agent::new(0, String::from("state of the agent"), ());
    ///
    /// agent.state_mut().truncate(5);
    /// assert_eq!(*agent.state(), "state");
    /// ```
    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    /// Get a reference to the agent's behavior.
    ///
    /// ```
    /// use tag_game::Agent;
    ///
    /// let mut agent = Agent::new(0, "state", 10);
    ///
    /// assert_eq!(*agent.behavior(), 10);
    /// ```
    pub const fn behavior(&self) -> &B {
        &self.behavior
    }

    /// Get a mutable reference to the agent's behavior.
    ///
    /// ```
    /// use std::mem;
    /// use tag_game::Agent;
    ///
    /// let mut agent = Agent::new(0, "state", 10);
    ///
    /// let old_behavior = mem::replace(agent.behavior_mut(), 20);
    /// assert_eq!(old_behavior, 10);
    /// assert_eq!(*agent.behavior(), 20);
    /// ```
    pub fn behavior_mut(&mut self) -> &mut B {
        &mut self.behavior
    }
}

#[cfg(test)]
mod tests {
    use crate::Agent;

    #[test]
    fn test_agent() {
        let mut agent = Agent::new(0, "0", 0);
        assert_eq!(agent.id(), 0);

        assert_eq!(*agent.state(), "0");
        *agent.state_mut() = "1";
        assert_eq!(*agent.state(), "1");

        assert_eq!(*agent.behavior(), 0);
        *agent.behavior_mut() = 1;
        assert_eq!(*agent.behavior(), 1);
    }
}
