use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use petgraph::prelude::DiGraphMap;

use crate::physical::ComputeNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Connection {
    pub source_node: NodeId,
    pub target_node: NodeId,
    pub source_id: u64,
    pub target_id: u64,
}

impl Connection {
    pub fn new(source_node: NodeId, target_node: NodeId, source_id: u64, target_id: u64) -> Self {
        Self {
            source_node,
            target_node,
            source_id,
            target_id,
        }
    }
}

#[derive(Clone)]
pub struct LogicalPlan {
    nodes: HashMap<NodeId, Arc<dyn ComputeNode>>,
    connections: Vec<Connection>,
}
