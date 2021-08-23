[![Test Status](https://github.com/TimDiekmann/TagGame/actions/workflows/test.yml/badge.svg?branch=main&event=push)](https://github.com/TimDiekmann/TagGame/actions/workflows/test.yml)
[![Docs](https://img.shields.io/static/v1?label=docs&message=main&color=5479ab)](https://timdiekmann.github.io/TagGame/tag_game/index.html)
[![codecov](https://codecov.io/gh/TimDiekmann/TagGame/branch/main/graph/badge.svg?token=RW5JNUBCXQ)](https://codecov.io/gh/TimDiekmann/TagGame)

Tag, You’re It!
===============

A simple agent-based engine to simulate the game “Tag”.

**Tag** is a simple game, I played in my childhood:
- There is some number of players (each of which is represented as an agent).
- At least one player is "It" and pursues the other players.
- When a player who is "It" gets close enough to another player, they can "Tag" that player. This is typically done by touching that player and saying "Tag!".
- A player who has been tagged becomes "It", and the tagging player is no longer “It”.
- No tag-backs: the player who is "It" may not tag the player who most recently tagged them.

Please find the source code for the game simulation in the [examples directory].



The agent-based engine
======================

The heart of this crate is the [`Simulation`]. It keeps track of all agents, its states and the globally shared state.

A [`Simulation`] is created with the global state [`World`]. This state is shared and accessible from all [`Agent`]s. An [`Agent`] can be added to the simulation using [`Simulation::add_agent()`]. Every [`Agent`] is associated with an own state, whose initial status has to be passed upon creation.

[`Simulation`]: https://timdiekmann.github.io/TagGame/tag_game/struct.Simulation.html
[`World`]: https://timdiekmann.github.io/TagGame/tag_game/trait.World.html
[`Agent`]: https://timdiekmann.github.io/TagGame/tag_game/trait.Agent.html
[`Simulation::add_agent()`]: https://timdiekmann.github.io/TagGame/tag_game/struct.Simulation.html#method.add_agent

Simulation
----------

To begin the simulation, the simulation can be advanced by one tick with [`Simulation::update()`]. When updating the simulation, [`Agent::on_update()`] is called for every agent, given him the possibility to act based on their current state, the global state and other agents currently present in the simulation, and mutate it’s state.

When all agents are updated, the world state is updated via [`World::update()`]. Unlike the agents, the [`World`] can mutate all states, including the global one.

[`Simulation::update()`]: https://timdiekmann.github.io/TagGame/tag_game/struct.Simulation.html#method.update
[`Agent::on_update()`]: https://timdiekmann.github.io/TagGame/tag_game/trait.Agent.html#method.on_update
[`World::update()`]: https://timdiekmann.github.io/TagGame/tag_game/trait.World.html#method.update

Examples
--------

To start a simulation, a world and an agent has to be defined:

```rust
#[derive(Clone)]
struct MyAgent {
    my_private_data: bool,
}

struct MyWorld {
    my_global_state: usize
}
```

Optionally, you can also define a state:

```rust
struct MyState {
    my_per_agent_state: &'static str,
}
```

Now, [`Agent`] and [`World`] has to be implemented. The state does not have an extra trait defined, as long as it is [`Send`] and [`Sync`].

[`Send`]: https://doc.rust-lang.org/core/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/core/marker/trait.Sync.html

```rust
use tag_game::{Agent, World};

impl Agent for MyAgent {
    type State = MyState;
    type World = MyWorld;

    fn on_creation(&self, id: usize, state: &mut MyState, world: &MyWorld) {
        println!(
            "I have been created with the id {} and the state {} in the world {}",
            id, state.my_per_agent_state, world.my_global_state
        );
    }

    fn on_update(
        &self,
        id: usize,
        state: &mut MyState,
        world: &MyWorld,
        _population: &[(Self, MyState)]
    ) {
        println!(
            "I have been updated with the id {} and the state {} in the world {}",
            id, state.my_per_agent_state, world.my_global_state
        );
    }
}

impl World<MyAgent> for MyWorld {
    fn update(&mut self, _agents: &mut [(MyAgent, MyState)]) {
        println!("The global state have been updated");
    }
}
```

Now create the world, add it to the [`Simulation`], and add an agent:

```rust
use tag_game::Simulation;

let world = MyWorld { my_global_state: 4 };
let mut simulation = Simulation::new(world);

let agent = MyAgent { my_private_data: true };
let state = MyState { my_per_agent_state: "Hello Agent" };
simulation.add_agent(agent, state);
```

and finally, run the simulation:

```rust
simulation.update();
```

License
-------

TagGame is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE] and [LICENSE-MIT] for details.

[examples directory]: https://github.com/TimDiekmann/TagGame/tree/main/examples/tag
[LICENSE-MIT]: https://github.com/TimDiekmann/TagGame/tree/main/LICENSE-MIT
[LICENSE-APACHE]: https://github.com/TimDiekmann/TagGame/tree/main/LICENSE-APACHE
