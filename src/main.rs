use petgraph::Graph;
use std::collections::BinaryHeap;

mod types;
use self::types::{Role, Passenger, Coord, VertexProperty, OrderByPassenger};

mod cmd_tree;
use self::cmd_tree::{CmdTree, CmdNode};


#[derive(Debug)]
enum Choice {
    Dequeue(Role),
}

#[derive(Debug)]
struct Route {
    graph: Graph::<VertexProperty, ()>,
    capacity: usize, // 1, 2, or 3. I.e., how many passengers can fit in the car
    // board layout, basically a HxW and coordinates of holes we can't drive on. Also start and end spots

    // found solutions, so we don't bother the user with the same stuff
}

impl Route {
    fn make_pieces() -> Graph::<VertexProperty, ()> {
        // andromeda 14

        let mut g = Graph::<VertexProperty, ()>::new();

        // purple passengers
        g.add_node(VertexProperty {
            role: Role::Source,
            passenger: Passenger::Purple,
            coord: Coord::new(4, 0),
        });
        g.add_node(VertexProperty {
            role: Role::Source,
            passenger: Passenger::Purple,
            coord: Coord::new(8, 0),
        });
        g.add_node(VertexProperty {
            role: Role::Source,
            passenger: Passenger::Purple,
            coord: Coord::new(4, 10),
        });
        g.add_node(VertexProperty {
            role: Role::Source,
            passenger: Passenger::Purple,
            coord: Coord::new(8, 10),
        });

        // purple sinks
        g.add_node(VertexProperty {
            role: Role::Sink,
            passenger: Passenger::Purple,
            coord: Coord::new(4, 3),
        });
        g.add_node(VertexProperty {
            role: Role::Sink,
            passenger: Passenger::Purple,
            coord: Coord::new(8, 3),
        });
        g.add_node(VertexProperty {
            role: Role::Sink,
            passenger: Passenger::Purple,
            coord: Coord::new(4, 7),
        });
        g.add_node(VertexProperty {
            role: Role::Sink,
            passenger: Passenger::Purple,
            coord: Coord::new(8, 7),
        });

        // orange passengers
        g.add_node(VertexProperty {
            role: Role::Source,
            passenger: Passenger::Orange,
            coord: Coord::new(2, 4),
        });
        g.add_node(VertexProperty {
            role: Role::Source,
            passenger: Passenger::Orange,
            coord: Coord::new(2, 5),
        });
        g.add_node(VertexProperty {
            role: Role::Source,
            passenger: Passenger::Orange,
            coord: Coord::new(8, 5),
        });

        // orange sinks
        g.add_node(VertexProperty {
            role: Role::Sink,
            passenger: Passenger::Orange,
            coord: Coord::new(4, 5),
        });
        g.add_node(VertexProperty {
            role: Role::Sink,
            passenger: Passenger::Orange,
            coord: Coord::new(6, 3),
        });
        g.add_node(VertexProperty {
            role: Role::Sink,
            passenger: Passenger::Orange,
            coord: Coord::new(6, 7),
        });

        g
    }

    // XXX maybe a builder pattern to take in the various components
    // [Nodes], capacity, heigh, width
    // verify each node coord fits in HxW
    pub fn new() -> Route {
        Route {
            graph: Route::make_pieces(),
            capacity: 1,
        }
    }

    // DecisionTree -
    // Initial state: source_q, sink_q, [] /* empty car */, 

    pub fn find_route(&mut self) -> () {
        //self.graph.clear_edges();

        // build up priority queues
        let mut sources = BinaryHeap::new();
        let mut sinks = BinaryHeap::new();
        for v in self.graph.raw_nodes() {
            match v.weight.role {
                Role::Source => sources.push(OrderByPassenger::new(v)),
                Role::Sink => sinks.push(OrderByPassenger::new(v)),
            };
        }

        let mut cmds = CmdTree::new();
        let root = CmdNode::new(sources, sinks);
        cmds.new_node(root);

        // need to figure out how to preserve decisions if capacity > 1
    }
}


fn main() {
    let mut r = Route::new();

    let soln = r.find_route();

    println!("{:?}", r);
}
