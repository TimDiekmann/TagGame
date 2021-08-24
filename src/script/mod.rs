mod lua;

use serde::{Deserialize, Serialize};

pub use self::lua::LuaScriptHost;

#[allow(clippy::missing_errors_doc)]
pub trait ScriptHost<S: Serialize, W: Serialize> {
    type ScriptContext;
    type Error;

    /// Creates a new script context

    fn create_context(world: &W) -> Result<Self::ScriptContext, Self::Error>;

    /// Add a new behavior for agents returning the identifier for this behavior.
    ///
    /// The behavior can be specified at [`ScriptHost::add_agent`] or can be reloaded with
    /// [`ScriptHost::update_agent_behavior`].
    fn add_agent_behavior(
        context: &mut Self::ScriptContext,
        source: &str,
    ) -> Result<u32, Self::Error>;

    /// Add a new behavior for the global world update returning the identifier for this behavior.
    ///
    /// If this function is called multiple times, all behavior will be executed after updating the
    /// agents.
    fn add_world_behavior(
        context: &mut Self::ScriptContext,
        source: &str,
    ) -> Result<u32, Self::Error>;

    /// Reloads an agent behavior specified by the id returned from [`ScriptHost::add_agent_behavior`].
    fn update_agent_behavior(
        context: &mut Self::ScriptContext,
        id: u32,
        source: &str,
    ) -> Result<(), Self::Error>;

    /// Reloads a world behavior specified by the id returned from [`ScriptHost::add_world_behavior`].
    fn update_world_behavior(
        context: &mut Self::ScriptContext,
        id: u32,
        source: &str,
    ) -> Result<(), Self::Error>;

    /// Adds an agent with the specified behavior returned from [`ScriptHost::add_agent_behavior`].
    fn add_agent(
        context: &mut Self::ScriptContext,
        behavior: u32,
        state: S,
    ) -> Result<usize, Self::Error>;

    /// Updates all agent and world states.
    fn update(context: &mut Self::ScriptContext) -> Result<(), Self::Error>;

    /// Returns the current world.
    fn world<'de>(context: &'de Self::ScriptContext) -> Result<W, Self::Error>
    where
        W: Deserialize<'de>;

    /// Returns a list of all agents.
    fn agents<'de>(context: &'de Self::ScriptContext) -> Result<Vec<S>, Self::Error>
    where
        S: Deserialize<'de>;
}
