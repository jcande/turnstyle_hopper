use std::collections::BinaryHeap;

use indextree::Arena;

use crate::types::{OrderByPassenger};

pub struct CmdNode<'a> {
    pub car: Vec<OrderByPassenger<'a>>,   // assert(role == Source)
    pub source_q: BinaryHeap<OrderByPassenger<'a>>,
    pub sink_q: BinaryHeap<OrderByPassenger<'a>>,
}

impl<'a> CmdNode<'a> {
    pub fn new(srcs: BinaryHeap<OrderByPassenger<'a>>, sinks: BinaryHeap<OrderByPassenger<'a>>) -> CmdNode<'a> {
        CmdNode {
            car: Vec::new(),
            source_q: srcs,
            sink_q: sinks,
        }
    }
}

pub type CmdTree<'a> = Arena<CmdNode<'a>>;
