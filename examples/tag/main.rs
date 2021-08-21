#![allow(unused)]

use serde::{Deserialize, Serialize};

use tag_game::{Agent, Behavior, Simulation};

/// The state, if an agent is tagged.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
enum Tag {
    /// The agent is currently "It"
    It,
    /// The agent recently was "It"
    Recent,
    /// The agent can be tagged by "It"
    None,
}

/// The current State an agent.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct AgentState {
    pub tag: Tag,
}

/// Prints to the console as soon as an event occurs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct PrintBehavior;

impl Behavior for PrintBehavior {
    type State = AgentState;
    type World = ();

    fn on_creation(&self, agent: &Agent<Self::State, Self>, world: &Self::World) {
        println!(
            "Agent created. id: {}, tag: {:?}",
            agent.id(),
            agent.state().tag
        );
    }

    fn on_deletion(&self, agent: &Agent<Self::State, Self>) {
        println!(
            "Agent removed. id: {}, tag: {:?}",
            agent.id(),
            agent.state().tag
        );
    }
}

fn main() {
    let mut simulation = Simulation::new(());

    let it_state = AgentState { tag: Tag::It };
    let no_state = AgentState { tag: Tag::None };

    simulation.add_agent(it_state, PrintBehavior);
    simulation.add_agent(no_state, PrintBehavior);
    simulation.add_agent(no_state, PrintBehavior);
}
