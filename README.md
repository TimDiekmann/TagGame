[![Test Status](https://github.com/TimDiekmann/TagGame/actions/workflows/test.yml/badge.svg?branch=main&event=push)](https://github.com/TimDiekmann/TagGame/actions/workflows/test.yml)
[![Docs](https://img.shields.io/static/v1?label=docs&message=main&color=5479ab)](https://timdiekmann.github.io/TagGame/tag_game/index.html)
[![codecov](https://codecov.io/gh/TimDiekmann/TagGame/branch/main/graph/badge.svg?token=RW5JNUBCXQ)](https://codecov.io/gh/TimDiekmann/TagGame)

Tag, You’re It!
---------------

A simple engine for running an agent-based model in Rust which runs a simulation where agents play the game “Tag”.

**Tag** is a simple game, I played in my childhood:
- There is some number of players (each of which is represented as an agent).
- At least one player is "It" and pursues the other players.
- When a player who is "It" gets close enough to another player, they can "Tag" that player. This is typically done by touching that player and saying "Tag!".
- A player who has been tagged becomes "It", and the tagging player is no longer “It”.
- No tag-backs: the player who is "It" may not tag the player who most recently tagged them.


License
-------

TagGame is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](https://github.com/TimDiekmann/TagGame/blob/master/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/TimDiekmann/TagGame/blob/master/LICENSE-MIT) for details.
