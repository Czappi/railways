pub mod execution_flow;
pub mod list;
pub mod node;
pub mod value;

use std::{
    any::TypeId,
    hash::{DefaultHasher, Hash, Hasher},
};

pub use node::Node;
use typed_builder::TypedBuilder;
pub use value::Value;

use crate::dsl::execution_flow::ExecutionFlow;

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

    fn targets(&'a self) -> Vec<Box<dyn LogicalTarget<'a> + 'a>>;
}

pub trait FlowControl<'a>: LogicalNode<'a> + Sized {
    const INDEX: usize;

    fn after(self, after: &'a dyn LogicalNode<'a>) -> Self;

    fn execution_flow(&'a self) -> ExecutionFlow<'a> {
        ExecutionFlow::new(self, Self::INDEX)
    }
}

pub trait LogicalSource<'a>: Identify {
    fn source(&'a self) -> &'a dyn LogicalNode<'a>;

    fn index(&self) -> usize;

    fn info(&self) -> PinInformation;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TypedBuilder)]
pub struct PinInformation {
    #[builder(default, setter(strip_option))]
    ty: Option<TypeId>,

    #[builder(default, setter(strip_option, into))]
    ty_name: Option<String>,

    #[builder(default, setter(strip_option, into))]
    ty_value: Option<String>,

    #[builder(default, setter(strip_option, into))]
    name: Option<String>,

    #[builder(default, setter(strip_option, into))]
    required: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State<T> {
    id: u64,
    pub value: T,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: rand::random(),
            value,
        }
    }
}

pub trait LogicalTarget<'a>: Identify {
    fn source(&'a self) -> Option<&'a dyn LogicalSource<'a>>;

    fn index(&self) -> usize;

    fn info(&self) -> PinInformation;

    fn is_required(&self) -> bool;

    fn boxed(self) -> Box<dyn LogicalTarget<'a> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

#[cfg(test)]
mod test {

    use crate::dsl::{
        value::ValueTarget, Identify, LogicalNode, LogicalSource, LogicalTarget, Node, State, Value,
    };

    #[derive(PartialEq, Eq, Hash)]
    pub struct U64Node(u64);

    impl U64Node {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn value<'a>(&'a self) -> Value<'a, u64> {
            Value::new(self, 0)
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

        fn targets(&'a self) -> Vec<Box<dyn super::LogicalTarget<'a> + 'a>> {
            Vec::new()
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct TNode<'a> {
        id: Value<'a, u64>,
    }

    pub fn node<'a>(id: Value<'a, u64>) -> Node<'a, TNode<'a>> {
        Node::new(TNode { id })
    }

    impl<'a> LogicalNode<'a> for TNode<'a> {
        fn name(&self) -> String {
            "Node".to_owned()
        }

        fn sources(&'a self) -> Vec<Box<dyn super::LogicalSource<'a> + 'a>> {
            Vec::new()
        }

        fn targets(&'a self) -> Vec<Box<dyn super::LogicalTarget<'a> + 'a>> {
            vec![ValueTarget::<'a, _, true>::new(&self.id, 0).boxed()]
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct SNode<'a> {
        state: State<usize>,
        id: Value<'a, u64>,
    }

    pub fn snode<'a>(id: Value<'a, u64>) -> Node<'a, SNode<'a>> {
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

        fn targets(&'a self) -> Vec<Box<dyn super::LogicalTarget<'a> + 'a>> {
            vec![ValueTarget::<'a, _, true>::new(&self.id, 0).boxed()]
        }
    }

    #[test]
    fn test() {
        let u = u64(0);

        let node = node(u.value());

        assert_eq!(node.id.source().identity(), u.identity())
    }

    #[test]
    fn state_test() {
        let u = u64(0);

        let snode1 = snode(u.value());
        let snode2 = snode(u.value());

        assert_eq!(snode1.id.source().identity(), u.identity());
        assert_eq!(snode1.id.source().identity(), snode2.id.source().identity());
        assert_ne!(snode1.identity(), snode2.identity());
    }
}
