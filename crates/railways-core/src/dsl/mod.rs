use std::{
    any::TypeId,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

pub trait Identify {
    fn identity(&self) -> u64;
}

impl<T: Hash> Identify for T {
    fn identity(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

pub trait LogicalNode<'a>: Identify {
    fn name(&self) -> String;

    fn sources(&'a self) -> Vec<Box<dyn LogicalSource<'a> + 'a>>;
}

pub trait LogicalSource<'a> {
    fn node(&'a self) -> &'a dyn LogicalNode<'a>;

    fn index(&self) -> usize;

    fn ty(&self) -> TypeId;
}

pub struct Node<'a, T: LogicalNode<'a> + ?Sized>(Box<T>, PhantomData<&'a T>);

impl<'a, T: LogicalNode<'a> + Sized> Node<'a, T> {
    pub fn new(node: T) -> Node<'a, T> {
        Node(Box::new(node), PhantomData)
    }

    pub fn node(&self) -> &'a dyn LogicalNode {
        self.0.as_ref()
    }
}

impl<'a, T: LogicalNode<'a>> Deref for Node<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub struct Source<'a, T> {
    pub node: &'a dyn LogicalNode<'a>,
    index: usize,
    p: PhantomData<T>,
}

impl<'a, T> Hash for Source<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.node.identity());
        Hash::hash(&self.index, state);
        Hash::hash(&self.p, state);
    }
}

impl<'a, T: Eq> Eq for Source<'a, T> {}

impl<'a, T: PartialEq> PartialEq for Source<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.node.identity() == other.node.identity()
            && self.index == other.index
            && self.p == other.p
    }
}

impl<'a, T: 'static> LogicalSource<'a> for Source<'a, T> {
    fn node(&'a self) -> &'a dyn LogicalNode<'a> {
        self.node
    }

    fn index(&self) -> usize {
        self.index
    }

    fn ty(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl<'a, T> Source<'a, T> {
    pub fn new(node: &'a dyn LogicalNode<'a>, index: usize) -> Self {
        Self {
            node,
            index,
            p: PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State<T> {
    id: uuid::Uuid,
    pub value: T,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            value,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::dsl::{Identify, LogicalNode, Node, Source, State};

    #[derive(PartialEq, Eq, Hash)]
    pub struct U64Node(u64);

    impl U64Node {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn value<'a>(&'a self) -> Source<'a, u64> {
            Source::new(self, 0)
        }
    }

    pub fn u64<'a>(value: u64) -> U64Node {
        U64Node::new(value)
    }

    impl<'a> LogicalNode<'a> for U64Node {
        fn name(&self) -> String {
            self.0.to_string()
        }

        fn sources(&'a self) -> Vec<Box<dyn super::LogicalSource<'a> + 'a>> {
            vec![Box::new(self.value())]
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct TNode<'a> {
        id: Source<'a, u64>,
    }

    pub fn node<'a>(id: Source<'a, u64>) -> Node<'a, TNode<'a>> {
        Node::new(TNode { id })
    }

    impl<'a> LogicalNode<'a> for TNode<'a> {
        fn name(&self) -> String {
            "Node".to_owned()
        }

        fn sources(&'a self) -> Vec<Box<dyn super::LogicalSource<'a> + 'a>> {
            Vec::new()
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct SNode<'a> {
        state: State<usize>,
        id: Source<'a, u64>,
    }

    pub fn snode<'a>(id: Source<'a, u64>) -> Node<'a, SNode<'a>> {
        Node::new(SNode {
            id,
            state: State::new(69),
        })
    }

    impl<'a> LogicalNode<'a> for SNode<'a> {
        fn name(&self) -> String {
            "Node".to_owned()
        }

        fn sources(&'a self) -> Vec<Box<dyn super::LogicalSource<'a> + 'a>> {
            Vec::new()
        }
    }

    #[test]
    fn test() {
        let u = u64(69);

        let node = node(u.value());

        assert_eq!(node.id.node.name(), String::from("69"));

        assert_eq!(node.id.node.identity(), u.identity())
    }

    #[test]
    fn state_test() {
        let u = u64(69);

        let snode1 = snode(u.value());
        let snode2 = snode(u.value());

        assert_eq!(snode1.id.node.name(), String::from("69"));

        assert_eq!(snode1.id.node.identity(), u.identity());
        assert_eq!(snode1.id.node.identity(), snode2.id.node.identity());
        assert_ne!(snode1.identity(), snode2.identity());
    }
}
