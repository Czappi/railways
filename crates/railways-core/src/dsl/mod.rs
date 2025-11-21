pub mod execution_flow;
pub mod list;
pub mod node;
pub mod value;

use std::{
    any::TypeId,
    hash::{DefaultHasher, Hash, Hasher},
};

pub use node::Node;
use thiserror::Error;
use typed_builder::TypedBuilder;
pub use value::Value;

use crate::{dsl::execution_flow::ExecutionFlow, physical::ComputeNode};

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

#[derive(Error, Debug, Clone)]
pub enum PinConnectionError {
    #[error("Required source is missing: {0:?}")]
    MissingRequiredSource(u64),
    #[error("Connected source ({source_id}) return type ({source_type}) and target ({target_id}) type ({target_type}) are mismatched")]
    TypeMismatch {
        source_id: u64,
        target_id: u64,
        source_type: String,
        target_type: String,
    },
}

#[derive(Error, Debug)]
#[error("Required sources are missing: {0:?}")]
pub struct MissingRequiredSourcesError(pub Vec<u64>);

#[derive(Error, Debug)]
pub enum IntoComputeNodeError {
    #[error("Core nodes can't be converted into a ComputeNode")]
    Core,
    #[error("Collected errors during ")]
    PinConnection(Vec<PinConnectionError>),
}

pub trait LogicalNode<'a>: Identify {
    fn name(&self) -> String;

    fn outputs(&'a self) -> Vec<Box<dyn LogicalSource<'a> + 'a>>;

    fn inputs<'b>(&'b self) -> Vec<&'b dyn LogicalTarget<'a>>
    where
        'a: 'b;

    fn into_compute(self) -> Result<Box<dyn ComputeNode>, IntoComputeNodeError>;

    fn check_inputs(&'a self) -> Result<(), Vec<PinConnectionError>> {
        let errors = self
            .inputs()
            .iter()
            .map(|target| target.check())
            .filter_map(|res| res.err())
            .collect::<Vec<_>>();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub trait FlowControl<'a>: LogicalNode<'a> + Sized {
    const INDEX: usize;

    fn after(self, after: &'a dyn LogicalNode<'a>) -> Self;

    fn execution_flow(&'a self) -> ExecutionFlow<'a> {
        ExecutionFlow::new(self, "Execution flow", Self::INDEX)
    }
}

pub trait LogicalSource<'a>: Identify {
    fn node(&'a self) -> &'a dyn LogicalNode<'a>;

    fn index(&self) -> usize;

    fn info(&self) -> PinInformation;

    fn boxed_clone(&self) -> Box<dyn LogicalSource<'a> + 'a>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TypedBuilder)]
pub struct PinInformation {
    pub ty: TypeId,

    #[builder(default, setter(strip_option, into))]
    pub ty_name: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub ty_value: Option<String>,

    #[builder(setter(into))]
    pub name: String,

    #[builder(default, setter(strip_option, into))]
    pub required: Option<bool>,
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
    fn source<'b>(&'b self) -> Option<&'b dyn LogicalSource<'a>>
    where
        'a: 'b;

    fn index(&self) -> usize;

    fn info(&self) -> PinInformation;

    fn is_required(&self) -> bool;

    fn check(&self) -> Result<(), PinConnectionError> {
        match (self.is_required(), self.source().as_ref().is_some()) {
            (false, true) | (true, true) => match self
                .source()
                .as_ref()
                .filter(|s| s.info().ty != self.info().ty)
            {
                Some(source) => Err(PinConnectionError::TypeMismatch {
                    source_id: source.identity(),
                    target_id: self.identity(),
                    source_type: source.info().ty_name.unwrap_or_default(),
                    target_type: self.info().ty_name.unwrap_or_default(),
                }),
                None => Ok(()),
            },
            (true, false) => Err(PinConnectionError::MissingRequiredSource(self.identity())),
            (false, false) => Ok(()),
        }
    }

    fn boxed_clone(&self) -> Box<dyn LogicalTarget<'a> + 'a>;
}

pub trait IntoLogicalTarget<'a, const REQUIRED: bool>: LogicalSource<'a> {
    type Target: LogicalTarget<'a>;

    fn into_target(&self, name: &'a str, index: usize) -> Self::Target;
}

#[cfg(test)]
mod test {

    use crate::dsl::{
        value::ValueTarget, Identify, IntoLogicalTarget, LogicalNode, LogicalSource, LogicalTarget,
        Node, State, Value,
    };

    #[derive(PartialEq, Eq, Hash)]
    pub struct U64Node(u64);

    impl U64Node {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn value<'a>(&'a self) -> Value<'a, u64> {
            Value::new(self, "u64", 0)
        }
    }

    pub fn u64<'a>(value: u64) -> U64Node {
        U64Node::new(value)
    }

    impl<'a> LogicalNode<'a> for U64Node {
        fn name(&self) -> String {
            self.0.to_string()
        }

        fn outputs(&'a self) -> Vec<Box<dyn LogicalSource<'a> + 'a>> {
            vec![Box::new(self.value())]
        }

        fn inputs<'b>(&'b self) -> Vec<&'b dyn LogicalTarget<'a>>
        where
            'a: 'b,
        {
            Vec::new()
        }

        fn into_compute(
            self,
        ) -> Result<Box<dyn crate::physical::ComputeNode>, super::IntoComputeNodeError> {
            todo!()
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct TNode<'a> {
        id: ValueTarget<'a, u64, true>,
    }

    pub fn node<'a>(id: Value<'a, u64>) -> Node<'a, TNode<'a>> {
        Node::<'a>::new(TNode::<'a> {
            id: id.into_target("id", 0),
        })
    }

    impl<'a> LogicalNode<'a> for TNode<'a> {
        fn name(&self) -> String {
            "Node".to_owned()
        }

        fn outputs(&'a self) -> Vec<Box<dyn super::LogicalSource<'a> + 'a>> {
            Vec::new()
        }

        fn inputs<'b>(&'b self) -> Vec<&'b dyn LogicalTarget<'a>>
        where
            'a: 'b,
        {
            vec![&self.id as &'b dyn LogicalTarget<'a>]
        }

        fn into_compute(
            self,
        ) -> Result<Box<dyn crate::physical::ComputeNode>, super::IntoComputeNodeError> {
            todo!()
        }
    }

    fn inputs<'a, 'b>(node: &'a TNode<'a>) -> Vec<&'b dyn super::LogicalTarget<'a>>
    where
        'a: 'b,
    {
        vec![&node.id as &'b dyn LogicalTarget<'a>]
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct SNode<'a> {
        state: State<usize>,
        id: ValueTarget<'a, u64, true>,
    }

    pub fn snode<'a>(id: Value<'a, u64>) -> Node<'a, SNode<'a>> {
        Node::new(SNode {
            id: id.into_target("id", 0),
            state: State::new(69),
        })
    }

    impl<'a> LogicalNode<'a> for SNode<'a> {
        fn name(&self) -> String {
            "Node".to_owned()
        }

        fn outputs(&'a self) -> Vec<Box<dyn super::LogicalSource<'a> + 'a>> {
            Vec::new()
        }

        fn inputs<'b>(&'b self) -> Vec<&'b dyn LogicalTarget<'a>>
        where
            'a: 'b,
        {
            vec![&self.id as &'b dyn LogicalTarget<'a>]
        }

        fn into_compute(
            self,
        ) -> Result<Box<dyn crate::physical::ComputeNode>, super::IntoComputeNodeError> {
            todo!()
        }
    }

    #[test]
    fn test() {
        let u = u64(0);

        let node = node(u.value());

        assert_eq!(node.id.source().unwrap().node().identity(), u.identity())
    }

    #[test]
    fn state_test() {
        let u = u64(0);

        let snode1 = snode(u.value());
        let snode2 = snode(u.value());

        assert_eq!(snode1.id.source().unwrap().node().identity(), u.identity());
        assert_eq!(
            snode1.id.source().unwrap().node().identity(),
            snode2.id.source().unwrap().node().identity()
        );
        assert_ne!(snode1.identity(), snode2.identity());
    }
}
