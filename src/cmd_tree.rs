use indextree::{Arena, NodeId};

use crate::types::{VertexProperty};

#[derive(PartialEq, Debug)]
pub enum CmdNode {
    Root,
    Choose(VertexProperty),
}

pub type CmdTree = Arena<CmdNode>;
pub type CmdNodeId = NodeId;
