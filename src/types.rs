use std::cmp::Ordering;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Role {
    Source,
    Sink,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Passenger {
    Purple,
    Orange,
    Smelly,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x,
            y: y,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct VertexProperty {
    pub role: Role,
    pub passenger: Passenger,
    pub coord: Coord
}

// Pipe through ordering to Passenger for our priority queue
#[derive(Debug)]
pub struct OrderByPassenger {
    pub data: VertexProperty,
}
impl OrderByPassenger {
    pub fn new(data: VertexProperty) -> OrderByPassenger {
        OrderByPassenger {
            data: data,
        }
    }
}
impl Ord for OrderByPassenger {
    fn cmp(&self, other: &Self) -> Ordering {
        fn as_usize(passenger: &Passenger) -> usize {
            const LOWEST_PRIORITY: usize = 0;
            const DEFAULT_PRIORITY: usize = 1;

            // These values represent the passenger's priority. Smelly's always
            // gotta be last.
            match passenger {
                Passenger::Purple => DEFAULT_PRIORITY,
                Passenger::Orange => DEFAULT_PRIORITY,
                Passenger::Smelly => LOWEST_PRIORITY,
            }
        }

        let this = as_usize(&self.data.passenger);
        let that = as_usize(&other.data.passenger);

        this.cmp(&that)
    }
}
impl PartialOrd for OrderByPassenger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for OrderByPassenger {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
// If this isn't here then rust shit's its fucking pants
impl Eq for OrderByPassenger {}
