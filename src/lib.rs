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
//!     type State = u32;
//!     type World = ();
//! }
//!
//! // Create a simulation with an empty world state
//! let mut simulation = Simulation::new(());
//!
//! // Add some agents
//! let agent_id_1 = simulation.add_agent(EmptyAgent, 2);
//! let agent_id_2 = simulation.add_agent(EmptyAgent, 3);
//!
//! // If desired, a list of all agents and its state can be retrieved
//! let mut agents = simulation.agents();
//! assert_eq!(agents[agent_id_1], (EmptyAgent, 2));
//! ```

mod agent;
mod simulation;
mod world;

pub use self::agent::Agent;
pub use self::simulation::Simulation;
pub use self::world::World;
