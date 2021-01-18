# Turnstyle Hopper
This was intended to be an AI for the great game [Cosmic Express](https://cosmicexpressgame.com/). It is not complete but portions of it are implemented.

# Approach
The overall approach is to solve the levels in multiple passes. On the first pass we generate a an abstract representation of the level visiting every "waypoint" (alien pickup and dropoff nodes) as mandatory. This step also ensures other properties are satisfied (e.g., the car will never be over capacity, we won't visit a dropoff node without the appropriately typed passenger, etc). The second pass, is the task of projecting this abstract path onto the discrete nodes of the actual level. The second task isn't done.

## Data structures
We only store "decisions" that we've made. These are structured in a tree and each layer of the tree corresponds to a particular decision. If we hit a failure condition (i.e., our constraints were violated) at a particular level then we never descend any further in the branch. By walking this tree we generate an abstract path along the level.

Currently the library [indextree](https://docs.rs/indextree/) is used.

# Comments
It's pretty slow on the example problem (andromeda 14). I haven't profiled it or anything so no clue where it is spending most of its time. The memory usage steadily grows but I didn't see it higher than 500mb (yeah, I know). It takes around five and a half minutes to finish. It outputs: "paths: 6429604, 725760", corresponding to total attempts, and "full" attempts respectively.

Anyway, I'm publishing this more out of a desire to mark this project as "done" than because anyone else will find it useful. The game is great and I still wish I could cheat at it. Oh well.
