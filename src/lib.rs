#![allow(clippy::must_use_candidate)]

//! # Examples
//!
//! To add agents to the simulation, [`Simulation::add_agent()`] and [`Simulation::remove_agent`]
//! may be called. Each agent requires a [`Behavior`], which has to be implemented.
//! ```
//! use tag_game::{Behavior, Simulation};
//!
//! #[derive(Debug, PartialEq)]
//! struct EmptyBehavior;
//!
//! impl Behavior for EmptyBehavior {
//!     type State = ();
//!     type World = ();
//! }
//!
//! // Create a simulation with an empty world state
//! let mut simulation = Simulation::new(());
//!
//! // Add some agents
//! let agent_id_1 = simulation.add_agent((), EmptyBehavior);
//! let agent_id_2 = simulation.add_agent((), EmptyBehavior);
//!
//! // If desired, an iterator over all agents can be retrieved
//! let mut agents = simulation.agents();
//! assert!(agents.find(|(id, _)| *id == agent_id_1).is_some());
//! // needed for dropck
//! drop(agents);
//!
//! // If an agent is not needed anymore, they can be removed
//! simulation.remove_agent(agent_id_1);
//! assert!(simulation.agents().find(|(id, _)| *id == agent_id_1).is_none());
//! ```

mod behavior;
mod simulation;

pub use self::behavior::Behavior;
pub use self::simulation::Simulation;
