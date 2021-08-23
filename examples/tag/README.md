Tag, You’re It!
===============

This is the implementation of the Tag game.

In order to run this example, run

```sh
cargo run --example tag --release
```

Configuration
=============

To customize the simulation a bit, some configuration can be made. If no configuration file exists, *config.json* will be crated from some default
values. 

```json
{
  "board": {
    "width": 50,
    "height": 50
  },
  "num_players": 10,
  "step": 1,
  "agents": {
    "untagged_deciding": {
      "start": 0.5,
      "end": 0.8
    },
    "tagged_deciding": {
      "start": 0.7,
      "end": 0.9
    },
    "untagged_speed_multiplied": {
      "start": 0.8,
      "end": 1.0
    },
    "tagged_speed_multiplied": {
      "start": 0.9,
      "end": 1.1
    }
  }
}
```

- `"board"` defines the dimension of the board
- `"num_players"` set the number of agents to generated
- `"steop"` is the step size how many updates will be done before drawing
  the current state to the terminal
- `"agents"` changes some behavior on the agents. A tagged agent("It") behaves
  different from other agents. Every value defines a range, from which a random
  value will be picked for each agent.

  - `"*_deciding"`: How likely the agent will run in the right direction. `0` means never and `1` means always.
  - `"*_speed_multiplied:`: The speed multiplier for moving around. A faster agent is unlikely to be tagged unless he is very bad at deciding.


Implementation
==============

For the terminology, please the the [crate level documentation](https://github.com/TimDiekmann/TagGame).

State
-----

The `AgentState` contains `Tag`, `Position`, and `Properties`. 

`Tag` describes, how the agent will behave and
how it's rendered. An agent is either

- `Tag::It`: The agent tries to tag other agents. Optionally, another agent id is stored along side. This corresponds to an agent, who "It" just tagged.
- `Tag::Recent`: The agent was recently tagged, so he neither can tag other agents, nor he can be tagged.
- `Tag::None`: The agennt can be tagged by "It".

`Position` describes the current position on the board and `Properties` are some attributes to change the behavior for agents.

World
-----

The `TagWorld` contains a board and the information, which agent is currently tagged. This isn't strictly needed to be stored in the world, however, updating
agent states is much simpler and faster this way. It also contains the information, which agend was most recently tagged.

When updating the world, the `current_it` is checked, if the agent has tagged another agent, thus the tag of `current_it` is `Tag::It(Some(id))` where `id` is the
agent who will become the new "It". `TagWorld.current_it` and `TagWorld.recent_it` are then updated appropriately.

Agent
-----

The `TagAgent` does not store any private data. At every update, first it checks, if `State.Tag` is correctly set, as the world state may have changed since the last tick.
Then, the different agents try to behave correctly, depending on their `"deciding"` attribute. `Tag::It` tries to tag another agent, `Tag::Recent` just looking around and 
`Tag::None` tries to flee but also tries not to run away too far.

*main.rs*
---------

In *main.rs* everything is put together: The config is loaded, the agents are created, the world is generated and the simulation is set up. Then, a terminal is used
to print the states, so we can watch the agents running around. In the main loop, for different keys are listened, so the simulation can be have as we want. everytime, *t* is pressed, `Simulation::update` is called. *q*,  *ESC*, *^C*, and *^D* quits the simulation. If the board is larger than the terminal, *h*, *j*, *k*, *l* or the arrow keys
can be used to scroll the board.
