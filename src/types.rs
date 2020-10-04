use petgraph::graph::Node;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Role {
    Source,
    Sink,
}

// XXX somehow we need to define a numeric value for each enum so we can use it
// in our priority queue
#[derive(Debug, Eq)]
pub enum Passenger {
    Purple,
    Orange,
    Smelly,
}

impl Ord for Passenger {
    fn cmp(&self, other: &Self) -> Ordering {
        fn as_usize(passenger: &Passenger) -> usize {
            match passenger {
                Passenger::Purple => 1,
                Passenger::Orange => 1,
                Passenger::Smelly => 0,
            }
        }

        let this = as_usize(self);
        let that = as_usize(other);

        this.cmp(&that)
    }
}

impl PartialOrd for Passenger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Passenger {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug)]
pub struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x,
            y: y,
        }
    }
}

#[derive(Debug)]
pub struct VertexProperty {
    pub role: Role,
    pub passenger: Passenger,
    pub coord: Coord
}

// Pipe through ordering to Passenger for our priority queue
pub struct OrderByPassenger<'a> {
    data: &'a Node<VertexProperty>,
}
impl<'a> OrderByPassenger<'a> {
    pub fn new(data: &'a Node<VertexProperty>) -> OrderByPassenger {
        OrderByPassenger {
            data: data,
        }
    }
}
impl<'a> Ord for OrderByPassenger<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.weight.passenger.cmp(&other.data.weight.passenger)
    }
}
impl<'a> PartialOrd for OrderByPassenger<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.data.weight.passenger.cmp(&other.data.weight.passenger))
    }
}
impl<'a> PartialEq for OrderByPassenger<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data.weight.passenger == other.data.weight.passenger
    }
}
// If this isn't here then rust shit's its fucking pants
impl<'a> Eq for OrderByPassenger<'a> {}
