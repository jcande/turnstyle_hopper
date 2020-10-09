use std::collections::BinaryHeap;
use std::collections::HashSet;

mod types;
use self::types::{Role, Passenger, Coord, OrderByPassenger, VertexProperty};

mod cmd_tree;
use self::cmd_tree::{CmdTree, CmdNode, CmdNodeId};


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

    pub fn find_route(&mut self) -> () {
        // We should at least have a root node from the constructor
        assert!(!self.arena.is_empty());
        let mut current: indextree::NodeId = self.root;

        // Create an empty car.
        let mut car: HashSet<Passenger> = HashSet::with_capacity(self.capacity);

        let mut route_choices: HashSet<VertexProperty> = HashSet::new();
        while route_choices.len() != self.pieces.len() {
            //
            // Figure out what choices we've made in the past.
            //
            let mut past_choices: HashSet<VertexProperty> = HashSet::new();
            if let Some(first_of_kin) = &self.arena[current].first_child() {
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
                .difference(&route_choices)
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
                // We've exhausted all of our options. It's time to admit
                // defeat and give it up.
                // XXX This should ascend instead of break. We should have a
                // separate check to see if we're at the root, in which case we
                // can break then as we've exhausted all paths
                None => {
                    let parent: indextree::NodeId = self.arena[current]
                        .parent()
                        .expect("We should never hit the root");
                    let parent: &CmdNode = self.arena[parent]
                        .get();

                    match parent {
                        CmdNode::Choose(_) => {
                            todo!("implement ascend (e.g., current_route.step_back()!");
                            continue
                        },
                        CmdNode::Root => {
                            println!("impossible constraints! Bailing");
                            break
                        },
                    };
                },
            };
            let decision: indextree::NodeId = self.arena.new_node(CmdNode::Choose(destination.clone()));
            current.append(decision, &mut self.arena);
            let source: &CmdNode = &self.arena[current].get();

            //
            // Verify the constraints. We want to make sure that this choice is
            // valid. If we've made a valid choice, we'll descend a level and
            // attempt to make more valid choices.
            //

            if !car.contains(&destination.passenger) &&
                destination.role == Role::Sink
            {
                //
                // We must be dropping off someone before visiting a
                // destination.
                //
                continue;
            }

            if car.len() == self.capacity &&
                destination.role == Role::Source
            {
                //
                // We can't pick someone up if we have no more room.
                //
                continue;
            }

            /*
            {
                CmdNode::Choose(data) => data,
                _ => todo!("figure out how to handle not having a previous vertex uniformly..."),
            }
            */
            // is it planar?
            // is it dropping someone off?
            // is it picking someone up with enough room?

            //
            // Looks good! Enforce the decision.
            //
            // XXX we should probably verify .remove()/.insert() and stuff. So that would mean we need to return an error from this function...
            match destination.role {
                Role::Sink => {
                    // XXX does this remove them from 0..max? If not, we'll
                    // need to wrap it in code that does. Might be the wrong
                    // level of abstraction.
                    car.remove(&destination.passenger);
                },
                Role::Source => {
                    car.insert(destination.passenger.clone());
                },
            };
            route_choices.insert(destination.clone());
            current = decision;
        }
        println!("route: {:?}", route_choices);
        println!("smoke weed every day");

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


        ()
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
        coord: Coord::new(2, 5),
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

    let soln = r.find_route();

    //println!("{:?}", r);
}
