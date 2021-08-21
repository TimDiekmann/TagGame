#![allow(unused)]

use tag_game::Agent;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tag {
    It,
    Recent,
}

/// The current State an agent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct State {
    tag: Option<Tag>,
}

impl State {
    pub const fn tag(self) -> Option<Tag> {
        self.tag
    }
}

impl State {
    const fn new(tag: Option<Tag>) -> Self {
        Self { tag }
    }
}

fn main() {
    let it = Some(Tag::It);

    let tagged_state = State::new(it);
    println!("{:?}", tagged_state);

    let agent = Agent::new(0, tagged_state);
    println!("{:?}", agent);
}
