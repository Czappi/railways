use std::{
    any::{type_name, TypeId},
    hash::{Hash, Hasher},
};

use crate::dsl::{Identify, IntoLogicalTarget, LogicalNode, LogicalSource, LogicalTarget};

/// Execution marker type
pub struct Execution;

#[derive(Clone, Copy)]
pub struct ExecutionFlow<'a> {
    source: &'a dyn LogicalNode<'a>,
    name: &'a str,
    index: usize,
}

impl<'a, const REQUIRED: bool> IntoLogicalTarget<'a, REQUIRED> for ExecutionFlow<'a> {
    type Target = ExecutionFlowTarget<'a, REQUIRED>;

    fn into_target(&self, name: &'a str, index: usize) -> Self::Target {
        ExecutionFlowTarget::new(self.clone(), name, index)
    }
}

impl<'a> Hash for ExecutionFlow<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"ExecutionFlow");
        state.write(self.name.as_bytes());
        state.write_usize(self.index);
        state.write_u64(self.source.identity());
    }
}

impl<'a> ExecutionFlow<'a> {
    pub fn new(source: &'a dyn LogicalNode<'a>, name: &'a str, index: usize) -> Self {
        Self {
            source,
            name,
            index,
        }
    }
}

impl<'a> LogicalSource<'a> for ExecutionFlow<'a> {
    fn node(&'a self) -> &'a dyn LogicalNode<'a> {
        self.source
    }

    fn index(&self) -> usize {
        self.index
    }

    fn info(&self) -> super::PinInformation {
        super::PinInformation::builder()
            .ty(TypeId::of::<Execution>())
            .ty_name(type_name::<Execution>())
            .name(self.name)
            .build()
    }

    fn boxed_clone(&self) -> Box<dyn LogicalSource<'a> + 'a> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Copy)]
pub struct ExecutionFlowTarget<'a, const REQUIRED: bool> {
    source: Option<ExecutionFlow<'a>>,
    name: &'a str,
    index: usize,
}

impl<'a, const REQUIRED: bool> ExecutionFlowTarget<'a, REQUIRED> {
    pub fn new(source: impl Into<Option<ExecutionFlow<'a>>>, name: &'a str, index: usize) -> Self {
        Self {
            source: source.into(),
            name,
            index,
        }
    }
}

impl<'a, const REQUIRED: bool> LogicalTarget<'a> for ExecutionFlowTarget<'a, REQUIRED> {
    fn source<'b>(&'b self) -> Option<&'b dyn LogicalSource<'a>>
    where
        'a: 'b,
    {
        self.source.as_ref().map(|s| s as &'b dyn LogicalSource<'a>)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn info(&self) -> super::PinInformation {
        super::PinInformation::builder()
            .name(self.name)
            .ty(TypeId::of::<Execution>())
            .ty_name(type_name::<Execution>())
            .required(REQUIRED)
            .build()
    }

    fn is_required(&self) -> bool {
        REQUIRED
    }

    fn boxed_clone(&self) -> Box<dyn LogicalTarget<'a> + 'a> {
        Box::new(self.clone())
    }
}

impl<'a, const REQUIRED: bool> Hash for ExecutionFlowTarget<'a, REQUIRED> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"ExecutionFlowTarget");
        state.write(self.name.as_bytes());
        state.write_usize(self.index);
        state.write_u64(self.source.identity());
    }
}
