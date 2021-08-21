#![allow(clippy::must_use_candidate)]

//! # Examples
//!
//! To add agents to the simulation, [`Simulation::add_agent()`] and [`Simulation::remove_agent`]
//! may be called. Each agent requires a [`Behavior`], which has to be implemented.
//! ```
//! use tag_game::{Agent, Simulation};
//!
//! #[derive(Debug, PartialEq)]
//! struct EmptyAgent;
//!
//! impl Agent for EmptyAgent {
//!     type State = ();
//!     type World = ();
//! }
//!
//! // Create a simulation with an empty world state
//! let mut simulation = Simulation::new(());
//!
//! // Add some agents
//! let agent_id_1 = simulation.add_agent(EmptyAgent, ());
//! let agent_id_2 = simulation.add_agent(EmptyAgent, ());
//!
//! // If desired, an iterator over all agents can be retrieved
//! let mut agents = simulation.iter();
//! assert!(agents.find(|(id, _)| *id == agent_id_1).is_some());
//! // needed for dropck
//! drop(agents);
//!
//! // If an agent is not needed anymore, they can be removed
//! simulation.remove_agent(agent_id_1);
//! assert!(simulation.iter().find(|(id, _)| *id == agent_id_1).is_none());
//! ```

mod agent;
mod simulation;

pub use self::agent::Agent;
pub use self::simulation::Simulation;
