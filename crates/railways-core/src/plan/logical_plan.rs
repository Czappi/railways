use std::collections::{HashMap, HashSet};

use petgraph::prelude::DiGraphMap;

pub struct NodeId(pub u64);

pub struct Connection {
    pub source_id: u64,
    pub target_id: u64,
}

pub struct LogicalPlan {
    task_graph: DiGraphMap<NodeId, ()>,
    //nodes: HashMap<NodeId>,
}
