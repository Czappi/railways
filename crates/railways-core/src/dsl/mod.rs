use std::{marker::PhantomData, ops::Deref};

pub trait LogicalNode {
    fn name(&self) -> String;
}

pub struct Node<T: LogicalNode + ?Sized>(Box<T>, PhantomData<T>);

impl<T: LogicalNode + Sized> Node<T> {
    pub fn new(node: T) -> Node<T> {
        Node(Box::new(node), PhantomData)
    }
}

impl<T: LogicalNode> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub struct Source<'a, T> {
    pub node: &'a dyn LogicalNode,
    index: usize,
    p: PhantomData<T>,
}

impl<'a, T> Source<'a, T> {
    pub fn new(node: &'a dyn LogicalNode, index: usize) -> Self {
        Self {
            node,
            index,
            p: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::dsl::{LogicalNode, Node, Source};

    pub struct U64Node(u64);

    impl U64Node {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn value<'a>(&'a self) -> Source<'a, u64> {
            Source::new(self, 0)
        }
    }

    pub fn u64(value: u64) -> Node<U64Node> {
        Node::new(U64Node::new(value))
    }

    impl LogicalNode for U64Node {
        fn name(&self) -> String {
            self.0.to_string()
        }
    }

    pub struct TNode<'a> {
        id: Source<'a, u64>,
    }

    pub fn node<'a>(id: Source<'a, u64>) -> Node<TNode<'a>> {
        Node::new(TNode { id })
    }

    impl<'a> LogicalNode for TNode<'a> {
        fn name(&self) -> String {
            "Node".to_owned()
        }
    }

    #[test]
    fn test() {
        let u = u64(69);

        let node = node(u.value());

        assert_eq!(node.id.node.name(), String::from("69"));
    }
}
