use std::collections::HashSet;

use crate::types::{Role, Passenger, VertexProperty};
use crate::cmd_tree::{CmdTree, CmdNode};

// XXX This should probably be an internal structure used by memory tree. A
// caller interacts solely with memory tree and calls "get_route". Memory tree
// repeatedly steps/unsteps until it hits the "bottom" of the tree (i.e., finds
// a complete route). Memory tree then converts this into a Vec<VertexProperty>
// and returns it as a result. If nothing found, returns an error of some sort.
#[derive(Debug)]
pub struct PathState {
    // The current position in the memory tree.
    pub levels: Vec<indextree::NodeId>,

    // The current state of the car.
    car: HashSet<Passenger>,
    capacity: usize, // 1, 2, or 3. I.e., how many passengers can fit in the car

    // The set of choices we made that make up the current route.
    // XXX we probably want something like levels but storing VertexProperty so
    // we can rebuild the path simply.
    pub choices: HashSet<VertexProperty>
}

pub enum PathErr {
    InvalidMove,

    OutOfWaypoints,
}

impl PathState {
    pub fn new(root: indextree::NodeId, capacity: usize) -> PathState {
        PathState {
            levels: vec![root],

            car: HashSet::with_capacity(capacity),
            capacity: capacity,

            choices: HashSet::new(),
        }
    }

    pub fn get_path(&self, memories: &CmdTree) -> Vec<VertexProperty> {
        self.levels.clone()
            .into_iter()
            .skip(1)    // We begin at the Root but that isn't helpful now
            .map(|id: indextree::NodeId| {
                let node = memories[id].get();
                match node {
                    CmdNode::Choose(destination) => destination.clone(),
                    CmdNode::Root => panic!("We should never encounter Root"),
                }
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        // We start out on level 1 (CmdNode::Root), but 0 choices.
        assert!(self.choices.len()+1 == self.levels.len());

        self.choices.len()
    }

    // Current level in the tree
    pub fn current_level(&self) -> Option<&indextree::NodeId> {
        self.levels.last()
    }

    fn commit_step(&mut self, target: VertexProperty) {
        match target.role {
            Role::Sink => {
                self.car.remove(&target.passenger);
            },
            Role::Source => {
                self.car.insert(target.passenger.clone());
            },
        };

        self.choices.insert(target);
    }

    fn uncommit_step(&mut self, target: &VertexProperty) {
        self.choices.remove(&target);

        match target.role {
            Role::Sink => {
                self.car.insert(target.passenger.clone());
            },
            Role::Source => {
                self.car.remove(&target.passenger);
            },
        };
    }

    fn verify_step(&self, target: &VertexProperty) -> Result<(), PathErr> {
            //
            // Verify the constraints. We want to make sure that this choice is
            // valid. If we've made a valid choice, we'll descend a level and
            // attempt to make more valid choices.
            //

            if !self.car.contains(&target.passenger) &&
                target.role == Role::Sink
            {
                //
                // We must be dropping off someone before visiting a
                // destination.
                //
                Err(PathErr::InvalidMove)?;
            }

            if self.car.len() == self.capacity &&
                target.role == Role::Source
            {
                //
                // We can't pick someone up if we have no more room.
                //
                Err(PathErr::InvalidMove)?;
            }

            // TODO is it planar? Wrong layer?

            Ok(())
    }

    pub fn add_waypoint(&mut self, target: VertexProperty, rung: indextree::NodeId) -> Result<(), PathErr> {
        // If rung corresponds to CmdNode::Root then all of our logic explodes.
        // So... don't do that.

        self.verify_step(&target)?;

        self.commit_step(target);

        self.levels.push(rung);

        Ok(())
    }

    pub fn remove_waypoint<'a>(&mut self, memories: &'a CmdTree) -> Result<&'a CmdNode, PathErr> {
        // N.B., we should always have at least the initial CmdNode::Root.
        assert!(self.levels.len() >= 1);
        if self.levels.len() == 1 {
            return Err(PathErr::OutOfWaypoints);
        }

        let node_id = self.levels.pop()
                .expect("We should always have an item at this point");
        let node: &CmdNode = memories[node_id].get();
        let target = match node {
            CmdNode::Choose(destination) => destination,
            CmdNode::Root => panic!("We should not have Root in our path"),
        };

        uncommit_step();

        Ok(node)
    }
}
