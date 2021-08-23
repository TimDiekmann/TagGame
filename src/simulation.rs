#![allow(clippy::missing_errors_doc)]
use serde::{Deserialize, Serialize};

use crate::ScriptHost;

/// Keeps track of all [`Agent`]s, its states and the global state.
///
/// It's responsible to update the `Agent`s states based on their defined behavior.
///
/// Please see the [crate documentation][crate] for examples.
pub struct Simulation<S: Serialize, W: Serialize, SH: ScriptHost<S, W>> {
    script_context: SH::ScriptContext,
}

impl<S: Serialize, W: Serialize, SH: ScriptHost<S, W>> Simulation<S, W, SH> {
    /// Creates a simulation with the provided [`World`].
    ///
    /// The `World` is passed to the agents, when they get update.
    ///
    /// # Errors
    ///
    /// Returns an error if the script host fails to create a context
    pub fn new(world: W) -> Result<Self, SH::Error> {
        Ok(Self {
            script_context: SH::create_context(&world)?,
        })
    }

    /// Get a shared reference to the global state.
    pub fn world<'de>(&'de self) -> Result<W, SH::Error>
    where
        W: Deserialize<'de>,
    {
        SH::world(&self.script_context)
    }

    /// Returns a slice over all agents and its state added to the simulation.
    #[inline]
    pub fn agents<'de>(&'de self) -> Result<Vec<S>, SH::Error>
    where
        S: Deserialize<'de>,
    {
        SH::agents(&self.script_context)
    }

    #[inline]
    pub fn add_agent_behavior(&mut self, source: &str) -> Result<u32, SH::Error> {
        SH::add_agent_behavior(&mut self.script_context, source)
    }

    #[inline]
    pub fn add_world_behavior(&mut self, source: &str) -> Result<u32, SH::Error> {
        SH::add_world_behavior(&mut self.script_context, source)
    }

    #[inline]
    pub fn update_agent_behavior(&mut self, id: u32, source: &str) -> Result<(), SH::Error> {
        SH::update_agent_behavior(&mut self.script_context, id, source)
    }

    #[inline]
    pub fn update_world_behavior(&mut self, id: u32, source: &str) -> Result<(), SH::Error> {
        SH::update_world_behavior(&mut self.script_context, id, source)
    }

    /// Add a new agent with an initial to the simulation.
    ///
    /// This will call [`Agent::on_creation()`] when the agent was created.
    ///
    /// Returns a unique identifier for the created agent.
    #[allow(clippy::missing_errors_doc)]
    pub fn add_agent(&mut self, behavior: u32, state: S) -> Result<(), SH::Error> {
        SH::add_agent(&mut self.script_context, behavior, state)?;
        // S::invoke(&self.script_context, id)?;
        Ok(())
    }
}

impl<S: Serialize, W: Serialize, SH: ScriptHost<S, W>> Simulation<S, W, SH> {
    /// Advances the simulation by one tick.
    ///
    /// First, on every added Agent, [`Agent::on_update()`] is invoked in arbitrary order.
    /// This happens in parallel. Afterwards, the global state is updated by calling
    /// [`World::update()`].
    ///
    /// To every [`Agent`] it's current state is passed as unique reference. Also a list
    /// of all other agents is passed as shared reference. The list is updated once before
    /// the tick and every agent will receive the same list.
    ///
    /// When updating the global state, a mutable slice to all `Agent`s and its states
    /// are passed to [`World`].
    pub fn update(&mut self) -> Result<(), SH::Error> {
        SH::invoke(&mut self.script_context)
    }
}
