use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use petgraph::prelude::DiGraphMap;

use crate::{
    dsl::{LogicalNode, MissingRequiredSourcesError, PinConnectionError},
    physical::ComputeNode,
};

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

/// Logical representiation of the `LogicalNode` borrow tree
///
///
#[derive(Clone)]
pub struct LogicalPlan<'a> {
    nodes: HashMap<NodeId, &'a dyn LogicalNode<'a>>,
    connections: HashSet<Connection>,
    errors: Vec<PinConnectionError>,
}

impl<'a> LogicalPlan<'a> {
    pub fn new(root: &'a dyn LogicalNode<'a>) -> Self {
        let walker = BorrowTreeWalker::new(root);
        let mut connections = HashSet::new();
        let mut nodes = HashMap::new();
        let mut errors = Vec::new();

        for res in walker {
            match res {
                Ok((node, connection)) => {
                    nodes.insert(NodeId(node.identity()), node);
                    connections.extend(connection.iter());
                }
                Err((error, node, connection)) => {
                    nodes.insert(NodeId(node.identity()), node);
                    connections.extend(connection.iter());
                    errors.extend(error);
                }
            }
        }

        Self {
            nodes,
            connections,
            errors,
        }
    }
}

pub struct BorrowTreeWalker<'a> {
    stack: Vec<&'a dyn LogicalNode<'a>>,
}

impl<'a> BorrowTreeWalker<'a> {
    pub fn new(root: &'a dyn LogicalNode<'a>) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a> Iterator for BorrowTreeWalker<'a> {
    type Item = Result<
        (&'a dyn LogicalNode<'a>, HashSet<Connection>),
        (
            Vec<PinConnectionError>,
            &'a dyn LogicalNode<'a>,
            HashSet<Connection>,
        ),
    >;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let mut connections = HashSet::new();
            let mut errors = Vec::new();

            for target in node.inputs() {
                match target.check() {
                    Ok(_) => {
                        if let Some(target_source) = target.source() {
                            connections.insert(Connection::new(
                                NodeId(target_source.node().identity()),
                                NodeId(node.identity()),
                                target_source.identity(),
                                target.identity(),
                            ));

                            self.stack.push(target_source.node());
                        }
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                }
            }

            if errors.is_empty() {
                Some(Ok((node.clone(), connections)))
            } else {
                Some(Err((errors, node.clone(), connections)))
            }
        } else {
            None
        }
    }
}
