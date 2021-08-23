#![allow(clippy::must_use_candidate)]

//! A simple agent-based engine to simulate the game "Tag".
//!
//! Overview
//! ========
//!
//! The heart of this crate is the [`Simulation`]. It keeps track of all agents, its states and the
//! globally shared state.
//!
//! A `Simulation` is created with the global state [`World`]. This state is shared and accessible
//! from all [`Agent`]s. An `Agent` can be added to the simulation using
//! [`Simulation::add_agent()`]. Every `Agent` is associated with an own state, whose initial
//! status has to be passed upon creation.
//!
//! Simulation
//! ----------
//!
//! To begin the simulation, the simulation can be advanced by one tick with
//! [`Simulation::update()`]. When updating the simulation, [`Agent::on_update()`] is called for every
//! agent, given him the possibility to act based on their current state, the global state and
//! other agents currently present in the simulation, and mutate it's state.
//!
//! When all agents are updated, the world state is updated via [`World::update()`]. Unlike the agents,
//! the [`World`] can mutate all states, including the global one.
//!
//! Examples
//! --------
//!
//! To start a simulation, a world and an agent has to be defined:
//!
//! ```
//! #[derive(Clone)]
//! struct MyAgent {
//!     my_private_data: bool,
//! }
//!
//! struct MyWorld {
//!     my_global_state: usize
//! }
//! ```
//!
//! Optionally, you can also define a state:
//!
//! ```
//! struct MyState {
//!     my_per_agent_state: &'static str,
//! }
//! ```
//!
//! Now, [`Agent`] and [`World`] has to be implemented. The state does not have an extra
//! trait defined, as long as it is [`Send`] and [`Sync`].
//!
//!
//! ```
//! use tag_game::{Agent, World};
//!
//! # struct MyAgent;
//! # struct MyState { my_per_agent_state: &'static str };
//! # struct MyWorld { my_global_state: usize };
//! impl Agent for MyAgent {
//!     type State = MyState;
//!     type World = MyWorld;
//!
//!     fn on_creation(&self, id: usize, state: &mut MyState, world: &MyWorld) {
//!         println!(
//!             "I have been created with the id {} and the state {} in the world {}",
//!             id, state.my_per_agent_state, world.my_global_state
//!         );
//!     }
//!
//!     fn on_update(
//!         &self,
//!         id: usize,
//!         state: &mut MyState,
//!         world: &MyWorld,
//!         _population: &[(Self, MyState)]
//!     ) {
//!         println!(
//!             "I have been updated with the id {} and the state {} in the world {}",
//!             id, state.my_per_agent_state, world.my_global_state
//!         );
//!     }
//! }
//!
//! impl World<MyAgent> for MyWorld {
//!     fn update(&mut self, _agents: &mut [(MyAgent, MyState)]) {
//!         println!("The global state have been updated");
//!     }
//! }
//! ```
//!
//! Now create the world, add it to the [`Simulation`], and add an agent:
//!
//! ```
//! use tag_game::Simulation;
//! # use tag_game::{Agent, World};
//! # struct MyAgent { my_private_data: bool };
//! # struct MyState { my_per_agent_state: &'static str };
//! # struct MyWorld { my_global_state: usize };
//! # impl World<MyAgent> for MyWorld {}
//! # impl Agent for MyAgent { type State = MyState; type World = MyWorld; }
//!
//! let world = MyWorld { my_global_state: 4 };
//! let mut simulation = Simulation::new(world);
//!
//! let agent = MyAgent { my_private_data: true };
//! let state = MyState { my_per_agent_state: "Hello Agent" };
//! simulation.add_agent(agent, state);
//! ```
//!
//! and finally, run the simulation:
//!
//! ```
//! # use tag_game::{Simulation, Agent, World};
//! # #[derive(Clone)] struct MyAgent;
//! # impl Agent for MyAgent { type State = (); type World = (); }
//! # let mut simulation = Simulation::new(());
//! # simulation.add_agent(MyAgent, ());
//! # #[cfg(not(miri))]
//! simulation.update();
//! ```

mod agent;
mod simulation;
mod world;

pub use self::agent::Agent;
pub use self::simulation::Simulation;
pub use self::world::World;
