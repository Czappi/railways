use std::hash::{Hash, Hasher};

use crate::dsl::{LogicalNode, LogicalSource};

pub struct ExecutionFlow<'a> {
    source: &'a dyn LogicalNode<'a>,
}

impl<'a> Hash for ExecutionFlow<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"ExecutionFlow");
        state.write_u64(self.source.identity());
    }
}

impl<'a> ExecutionFlow<'a> {
    pub fn new(source: &'a dyn LogicalNode<'a>) -> Self {
        Self { source }
    }
}

impl<'a> LogicalSource<'a> for ExecutionFlow<'a> {
    fn source(&'a self) -> &'a dyn LogicalNode<'a> {
        self.source
    }

    fn index(&self) -> usize {
        0
    }

    fn info(&self) -> super::SourceInformation {
        super::SourceInformation::builder()
            .source_name("ExecutionFlow")
            .build()
    }
}
