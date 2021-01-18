use std::collections::BinaryHeap;
use std::collections::HashSet;

mod cmd_tree;
use self::cmd_tree::{CmdTree, CmdNode, CmdNodeId};

mod level;
use self::level::{Level};

mod path_state;
use self::path_state::{PathState};

mod types;
use self::types::{Role, Passenger, Coord, OrderByPassenger, VertexProperty};


#[derive(Debug)]
struct Route {
    level: Level,

    // board layout, basically a HxW and coordinates of holes we can't drive on. Also start and end spots

    // found solutions, so we don't bother the user with the same stuff
    // use the tree for that
    arena: CmdTree,
    root: CmdNodeId,

    path: PathState,
}

enum RouteErr {
    // Somehow we ascended beyond the root. This is definitely a bug.
    ImpossibleLevel,

    // Somehow path.pop_waypoint() failed. This is definitely a bug.
    UnableToBacktrack,

    // Couldn't solve the map
    Unsat
}

impl Route {
    // XXX maybe a builder pattern to take in the various components
    // [Nodes], capacity, heigh, width, start coord, end coord
    // verify each node coord fits in HxW
    pub fn new(level: Level) -> Route {
        // just for the time being
        assert!(level.capacity == 1);
        // assert!(level.pieces are unique);

        let mut cmds = CmdTree::new();
        let root = cmds.new_node(CmdNode::Root);

        Route {
            path: PathState::new(root, level.capacity),

            level: level,

            arena: cmds,
            root: root,
        }
    }

    pub fn path_count(&self) -> usize {
        self.arena.count()
    }

    //
    // XXX Current thinking. We need two data structures. "Route memory" and
    // "current route".
    // Route memory - this is the tree. It contains every choice we have ever
    // made.
    // Current route - this is the "state" that we use to evaluate our choices.
    // It represents a particular path down the tree (i.e., route memory). This
    // is the thing that we need to modify to "ascend".
    //

    pub fn find_route(&mut self) -> Result<(), RouteErr> {
        // We should at least have a root node from the constructor
        assert!(!self.arena.is_empty());

        while self.path.len() != self.level.pieces.len() {
            //
            // Figure out what choices we've made in the past.
            //
            let current = self.path.current_level()
                .ok_or(RouteErr::ImpossibleLevel)?;
            let mut past_choices: HashSet<VertexProperty> = HashSet::new();
            if let Some(first_of_kin) = &self.arena[*current].first_child() {
                for child_id in first_of_kin.following_siblings(&self.arena) {
                    if let CmdNode::Choose(passenger_type) = &self.arena[child_id].get() {
                        past_choices.insert(passenger_type.clone());
                    } else {
                        // We can only get here if we are hitting the root which we never should
                        assert!(false, "How did we get here?");
                    }
                }
            }

            //
            // Remove those past choices from our current set of options. While
            // we're at it, remove the choices that we've already made during
            // this particular route.
            //
            let remaining_choices: HashSet<VertexProperty> = self.level.pieces
                .difference(&past_choices)
                .cloned()
                .collect();
            let remaining_choices: HashSet<VertexProperty> = remaining_choices
                .difference(&self.path.choices)
                .cloned()
                .collect();
            let mut q: BinaryHeap<OrderByPassenger> = remaining_choices
                .into_iter()
                .map(|node| OrderByPassenger::new(node.clone()))
                .collect();

            //
            // We've made our choice. Add it to the ledger so we don't need to
            // make the same decision twice. If this decision turns out to be
            // ill advised, well we don't have to pursue it any further! But we
            // do have to remember that we chose it in the first place so we
            // don't repeat the mistake.
            //
            let destination: VertexProperty = match q.pop() {
                Some(wrapped) => wrapped.data,
                None => {
                    let parent = self.path.pop_waypoint(&self.arena)
                        .or(Err(RouteErr::UnableToBacktrack))?;
                    match parent {
                        CmdNode::Choose(_) => {
                            // We aren't yet at the root which means we haven't
                            // exhausted all possible options yet. Give it
                            // another try.
                            continue
                        },
                        CmdNode::Root => {
                            //println!("impossible constraints! Bailing. Should we return err instead?");
                            //break
                            return Err(RouteErr::Unsat)?;
                        },
                    };
                },
            };
            let decision: indextree::NodeId = self.arena.new_node(CmdNode::Choose(destination.clone()));
            current.append(decision, &mut self.arena);
            //let source: &CmdNode = &self.arena[current].get();

            //
            // Now we need to verify the current set of constraints. If they're
            // satisfiable then we will successfully add a new waypoint. If
            // not, then we'll have to make another decision. Either way, we'll
            // need to loop around at least one more time.
            //

            if self.path.push_waypoint(destination, decision).is_err() {
                // Unable to add the waypoint, try a new path!
                continue;
            }

            // deeper verification here?
            // Make a graph with all possible squares (i.e., the full board
            // minus the Role::Obstacles) all hooked up in valid ways and try
            // to plot a path given the new waypoint. If we can't do it, then
            // we pop the waypoint and continue.
            // A concern here is how do we know the "full" route that we select
            // is not the problem as oppose to the PathState route? I don't see
            // an obvious method to iterate over various full routes in an
            // attempt to get a working one. The problem with our full route
            // might only be evident many steps later, how can we know which
            // section to jiggle? Maybe checking for planarity is enough?
        }
        //let route = self.path.get_path(&self.arena);
        //println!("route: {:?}\n", route);


        // We've reached the leaves of the tree. Take a step back because if
        // this is not the right solution then we'll greedily try another path
        // at this same level.
        // XXX we should probably check for a weird error though (like .remove() failed or something)
        let _ = self.path.pop_waypoint(&self.arena);

        Ok(())
    }
}

fn make_level() -> Level {
    // andromeda 14

    let mut pieces = Vec::new();

    // purple passengers
    pieces.push(VertexProperty {
        role: Role::Source(Passenger::Purple),
        coord: Coord::new(4, 0),
    });
    pieces.push(VertexProperty {
        role: Role::Source(Passenger::Purple),
        coord: Coord::new(8, 0),
    });
    pieces.push(VertexProperty {
        role: Role::Source(Passenger::Purple),
        coord: Coord::new(4, 10),
    });
    pieces.push(VertexProperty {
        role: Role::Source(Passenger::Purple),
        coord: Coord::new(8, 10),
    });

    // purple sinks
    pieces.push(VertexProperty {
        role: Role::Sink(Passenger::Purple),
        coord: Coord::new(4, 3),
    });
    pieces.push(VertexProperty {
        role: Role::Sink(Passenger::Purple),
        coord: Coord::new(8, 3),
    });
    pieces.push(VertexProperty {
        role: Role::Sink(Passenger::Purple),
        coord: Coord::new(4, 7),
    });
    pieces.push(VertexProperty {
        role: Role::Sink(Passenger::Purple),
        coord: Coord::new(8, 7),
    });

    // orange passengers
    pieces.push(VertexProperty {
        role: Role::Source(Passenger::Orange),
        coord: Coord::new(2, 4),
    });
    pieces.push(VertexProperty {
        role: Role::Source(Passenger::Orange),
        coord: Coord::new(2, 6),
    });
    pieces.push(VertexProperty {
        role: Role::Source(Passenger::Orange),
        coord: Coord::new(8, 5),
    });

    // orange sinks
    pieces.push(VertexProperty {
        role: Role::Sink(Passenger::Orange),
        coord: Coord::new(4, 5),
    });
    pieces.push(VertexProperty {
        role: Role::Sink(Passenger::Orange),
        coord: Coord::new(6, 3),
    });
    pieces.push(VertexProperty {
        role: Role::Sink(Passenger::Orange),
        coord: Coord::new(6, 7),
    });

    /*
    A solution

    1     (2,  6): orange source {left}
    2     (6,  3)                {left}
    3     (0,  4): purple source {right}
    4     (3,  8)                {right}
    5     (0,  8): purple source {left}
    6     (4,  3)                {right}
    7     (2,  4): orange source {left}
    8     (4,  5)                {right}
    9     (4, 10): purple source {left}
    10    (4,  7)                {right}
    11    (8,  5): orange source {right}
    12    (6,  7)                {left}
    13    (8, 10): purple source {left}
    14    (8, 7)                 {right}

    left-left:      1
    right-right:    1
    left-right:     4
    right-left:     1
    */

    let level = Level::default()
        .with_pieces(pieces);

    level
}


fn main() {
    let mut r = Route::new(make_level());

    let mut full_length_paths = 0usize;
    loop {
        let soln = r.find_route();
        match soln {
            Err(_) => break,
            _ => { full_length_paths += 1; continue },
        };
    }

    println!("paths: {:?}, {}", r.path_count(), full_length_paths);
}
