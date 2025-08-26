use std::hash::{Hash, Hasher};

use crate::dsl::{Identify, LogicalNode, LogicalSource, LogicalTarget};

pub struct ExecutionFlow<'a> {
    source: &'a dyn LogicalNode<'a>,
    index: usize,
}

impl<'a> Hash for ExecutionFlow<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"ExecutionFlow");
        state.write_u64(self.source.identity());
        state.write_usize(self.index);
    }
}

impl<'a> ExecutionFlow<'a> {
    pub fn new(source: &'a dyn LogicalNode<'a>, index: usize) -> Self {
        Self { source, index }
    }
}

impl<'a> LogicalSource<'a> for ExecutionFlow<'a> {
    fn source(&'a self) -> &'a dyn LogicalNode<'a> {
        self.source
    }

    fn index(&self) -> usize {
        self.index
    }

    fn info(&self) -> super::PinInformation {
        super::PinInformation::builder()
            .name("ExecutionFlow")
            .build()
    }
}

pub struct ExecutionFlowTarget<'a, const REQUIRED: bool> {
    source: Option<&'a ExecutionFlow<'a>>,
    index: usize,
}

impl<'a, const REQUIRED: bool> ExecutionFlowTarget<'a, REQUIRED> {
    pub fn new(source: impl Into<Option<&'a ExecutionFlow<'a>>>, index: usize) -> Self {
        Self {
            source: source.into(),
            index,
        }
    }
}

impl<'a, const REQUIRED: bool> LogicalTarget<'a> for ExecutionFlowTarget<'a, REQUIRED> {
    fn source(&'a self) -> Option<&'a dyn LogicalSource<'a>> {
        self.source.map(|s| s as &'a dyn LogicalSource<'a>)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn info(&self) -> super::PinInformation {
        super::PinInformation::builder()
            .name("ExecutionFlowTarget")
            .required(REQUIRED)
            .build()
    }

    fn is_required(&self) -> bool {
        REQUIRED
    }
}

impl<'a, const REQUIRED: bool> Hash for ExecutionFlowTarget<'a, REQUIRED> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"ExecutionFlowTarget");
        state.write_u64(self.source.identity());
        state.write_usize(self.index);
    }
}
