#![allow(unused)]

use serde::{Deserialize, Serialize};

use tag_game::{Agent, Behavior, Simulation, State};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
enum Tag {
    It,
    Recent,
    None,
}

/// The current State an agent.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct AgentState {
    tag: Tag,
}

impl AgentState {
    pub const fn new(tag: Tag) -> Self {
        Self { tag }
    }

    pub const fn tag(self) -> Tag {
        self.tag
    }
}

impl State for AgentState {}

/// The current State an agent.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct WorldState;

impl State for WorldState {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct PrintBehavior;

impl Behavior for PrintBehavior {
    type State = AgentState;
    type World = WorldState;

    fn on_creation(&self, agent: &Agent<Self::State, Self>, world: &Self::World) {
        println!(
            "Agent created. id: {}, tag: {:?}",
            agent.id(),
            agent.state().tag()
        );
    }

    fn on_deletion(&self, agent: &Agent<Self::State, Self>) {
        println!(
            "Agent removed. id: {}, tag: {:?}",
            agent.id(),
            agent.state().tag()
        );
    }
}

fn main() {
    let mut simulation = Simulation::new(WorldState);

    let it_state = AgentState::new(Tag::It);
    let no_state = AgentState::new(Tag::None);

    simulation.add_agent(it_state, PrintBehavior);
    simulation.add_agent(no_state, PrintBehavior);
    simulation.add_agent(no_state, PrintBehavior);

    // let tagged_state = AgentState::new(it);
    // println!("{}", serde_json::to_string_pretty(&tagged_state).unwrap());

    // let agent = Agent::new(0, tagged_state, PrintBehavior);
    // println!("{}", serde_json::to_string_pretty(&agent).unwrap());
}
