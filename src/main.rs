use std::collections::BinaryHeap;
use std::collections::HashSet;

mod types;
use self::types::{Role, Passenger, Coord, OrderByPassenger, VertexProperty};

mod cmd_tree;
use self::cmd_tree::{CmdTree, CmdNode, CmdNodeId};

mod path_state;
use self::path_state::{PathState};


#[derive(Debug)]
struct Route {
    pieces: HashSet<VertexProperty>,

    capacity: usize, // 1, 2, or 3. I.e., how many passengers can fit in the car
    // board layout, basically a HxW and coordinates of holes we can't drive on. Also start and end spots

    // found solutions, so we don't bother the user with the same stuff
    // use the tree for that
    arena: CmdTree,
    root: CmdNodeId,
}

enum RouteErr {
    // Somehow we ascended beyond the root. This is definitely a bug.
    ImpossibleLevel,

    // Somehow path.remove_waypoint() failed. This is definitely a bug.
    UnableToBacktrack,

    // Couldn't solve the map
    Unsat
}

impl Route {
    // XXX maybe a builder pattern to take in the various components
    // [Nodes], capacity, heigh, width, start coord, end coord
    // verify each node coord fits in HxW
    pub fn new(pieces: Vec<VertexProperty>, capacity: usize) -> Route {
        // just for the time being
        assert!(capacity == 1);
        // assert!(pieces are unique);

        let mut cmds = CmdTree::new();
        let root = cmds.new_node(CmdNode::Root);

        Route {
            pieces: pieces.into_iter().collect(),

            capacity: 1,

            arena: cmds,
            root: root,
        }
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
        let mut path = PathState::new(self.root, self.capacity);
        while path.len() != self.pieces.len() {
            //
            // Figure out what choices we've made in the past.
            //
            let current = path.current_level()
                .ok_or(RouteErr::ImpossibleLevel)?;
            let mut past_choices: HashSet<VertexProperty> = HashSet::new();
            if let Some(first_of_kin) = &self.arena[*current].first_child() {
                for childId in first_of_kin.following_siblings(&self.arena) {
                    if let CmdNode::Choose(passenger_type) = &self.arena[childId].get() {
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
            let remaining_choices: HashSet<VertexProperty> = self.pieces
                .difference(&past_choices)
                .cloned()
                .collect();
            let remaining_choices: HashSet<VertexProperty> = remaining_choices
                .difference(&path.choices)
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
                    let parent = path.remove_waypoint(&self.arena)
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

            path.add_waypoint(destination, decision);
        }
        let route = path.as_path(&self.arena);
        println!("route: {:?}\n", route);

        /*
        // dfs on a ghost tree
        while src_q and sink_q are nonempty {
            // Descend the tree 1 layer
            cmd = make_choice()

            node = match cmd {
                dequeue (src) => src_q.dequeue()
                dequeue (sink) => sink_q.dequeue()
            }

            if (cmd, node) in current_layer.cmds {
                // already tried this
                rise() // ascend 1 layer
                continue;
            }

            verify_constraints(node).with_current_state();
            if (invalid) {
                current_layer.set(Invalid)
                rise() // ascend 1 layer
                contineu;
            }
        }

        // ascend back up the tree and restore the full state. We need to do
        // this in case this solution can't be projected onto the board.
        restore();

        return route
        */


        Ok(())
    }
}

fn make_pieces() -> Vec<VertexProperty> {
    // andromeda 14

    let mut pieces = Vec::new();

    // purple passengers
    pieces.push(VertexProperty {
        role: Role::Source,
        passenger: Passenger::Purple,
        coord: Coord::new(4, 0),
    });
    pieces.push(VertexProperty {
        role: Role::Source,
        passenger: Passenger::Purple,
        coord: Coord::new(8, 0),
    });
    pieces.push(VertexProperty {
        role: Role::Source,
        passenger: Passenger::Purple,
        coord: Coord::new(4, 10),
    });
    pieces.push(VertexProperty {
        role: Role::Source,
        passenger: Passenger::Purple,
        coord: Coord::new(8, 10),
    });

    // purple sinks
    pieces.push(VertexProperty {
        role: Role::Sink,
        passenger: Passenger::Purple,
        coord: Coord::new(4, 3),
    });
    pieces.push(VertexProperty {
        role: Role::Sink,
        passenger: Passenger::Purple,
        coord: Coord::new(8, 3),
    });
    pieces.push(VertexProperty {
        role: Role::Sink,
        passenger: Passenger::Purple,
        coord: Coord::new(4, 7),
    });
    pieces.push(VertexProperty {
        role: Role::Sink,
        passenger: Passenger::Purple,
        coord: Coord::new(8, 7),
    });

    // orange passengers
    pieces.push(VertexProperty {
        role: Role::Source,
        passenger: Passenger::Orange,
        coord: Coord::new(2, 4),
    });
    pieces.push(VertexProperty {
        role: Role::Source,
        passenger: Passenger::Orange,
        coord: Coord::new(2, 6),
    });
    pieces.push(VertexProperty {
        role: Role::Source,
        passenger: Passenger::Orange,
        coord: Coord::new(8, 5),
    });

    // orange sinks
    pieces.push(VertexProperty {
        role: Role::Sink,
        passenger: Passenger::Orange,
        coord: Coord::new(4, 5),
    });
    pieces.push(VertexProperty {
        role: Role::Sink,
        passenger: Passenger::Orange,
        coord: Coord::new(6, 3),
    });
    pieces.push(VertexProperty {
        role: Role::Sink,
        passenger: Passenger::Orange,
        coord: Coord::new(6, 7),
    });

    pieces
}


fn main() {
    let mut r = Route::new(make_pieces(), 1);

    loop {
        let soln = r.find_route();
        match soln {
            Err(_) => break,
            _ => continue,
        };
    }

    //println!("{:?}", r);
}
