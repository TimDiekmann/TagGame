/// The state, if an agent is tagged.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    /// The agent is currently "It"
    It,
    /// The agent recently was "It"
    Recent,
    /// The agent can be tagged by "It"
    None,
}

/// The current State an agent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AgentState {
    pub tag: Tag,
    pub position: [u16; 2],
}
