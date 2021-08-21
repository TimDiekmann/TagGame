#![allow(unused)]

use tag_game::{Agent, Simulation};

/// The state, if an agent is tagged.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tag {
    /// The agent is currently "It"
    It,
    /// The agent recently was "It"
    Recent,
    /// The agent can be tagged by "It"
    None,
}

/// The current State an agent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct AgentState {
    pub tag: Tag,
}

/// Prints to the console as soon as an event occurs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct PrintBehavior;

impl Agent for PrintBehavior {
    type State = AgentState;
    type World = ();

    fn on_creation(&self, id: u64, state: &mut Self::State, _world: &Self::World) {
        println!("Agent created. id: {}, tag: {:?}", id, state.tag);
    }

    fn on_deletion(&self, id: u64, state: &mut Self::State, _world: &Self::World) {
        println!("Agent removed. id: {}, tag: {:?}", id, state.tag);
    }

    fn on_update<'sim>(
        &'sim self,
        id: u64,
        state: &'sim mut Self::State,
        world: &'sim Self::World,
        population: impl Iterator<Item = (u64, &'sim Self::State)>,
    ) {
        println!(
            "UPDATE id: {}, state: {:?}, world: {:?}, population: {:?}",
            id,
            state.tag,
            world,
            population.map(|(id, _)| id).collect::<Vec<_>>()
        );
    }
}

fn main() {
    let mut simulation = Simulation::new(());

    let it_state = AgentState { tag: Tag::It };
    let no_state = AgentState { tag: Tag::None };

    simulation.add_agent(PrintBehavior, it_state);
    simulation.add_agent(PrintBehavior, no_state);
    simulation.add_agent(PrintBehavior, no_state);

    simulation.update();
}
