use std::hash::Hash;

use crate::{
    dsl::{
        value::ValueTarget, Identify, IntoLogicalTarget, LogicalNode, LogicalSource, LogicalTarget,
        Value,
    },
    plan::logical_plan::LogicalPlan,
};

struct Pipeline<'a> {
    name: String,
    outputs: Vec<&'a dyn crate::dsl::LogicalSource<'a>>,
    inputs: Vec<&'a dyn crate::dsl::LogicalTarget<'a>>,
    plan: LogicalPlan<'a>,
}

impl<'a> Pipeline<'a> {
    fn new<F, N>(name: String, f: F) -> Self
    where
        F: Fn(PipelineContext<'a>) -> N,
        N: LogicalNode<'a>,
    {
        let root = f(PipelineContext {
            source: &self,
            targets: Vec::new(),
        });

        Self {
            name,
            outputs,
            inputs,
            plan,
        }
    }
}

impl<'a> Hash for Pipeline<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);

        self.outputs
            .iter()
            .for_each(|s| state.write_u64(s.identity()));
        self.inputs
            .iter()
            .for_each(|t| state.write_u64(t.identity()));
    }
}

impl<'a> LogicalNode<'a> for Pipeline<'a> {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn outputs(&'a self) -> Vec<Box<dyn crate::dsl::LogicalSource<'a> + 'a>> {
        self.outputs.iter().map(|s| s.boxed_clone()).collect()
    }

    fn inputs<'b>(&'b self) -> Vec<&'b dyn crate::dsl::LogicalTarget<'a>>
    where
        'a: 'b,
    {
        self.inputs.iter().copied().collect()
    }

    fn into_compute(
        self,
    ) -> Result<Box<dyn crate::physical::ComputeNode>, crate::dsl::IntoComputeNodeError> {
        Err(crate::dsl::IntoComputeNodeError::Core)
    }
}

pub struct PipelineContext<'a> {
    source: &'a dyn LogicalNode<'a>,
    targets: Vec<Box<dyn crate::dsl::LogicalTarget<'a> + 'a>>,
}

impl<'a> PipelineContext<'a> {
    pub fn value<T: 'static>(&mut self, name: &'a str) -> Value<'a, T> {
        self.input::<_, true>(
            name,
            Value::<'a, T>::new(self.source, name, self.targets.len()),
        )
    }

    pub fn input<S, const REQUIRED: bool>(&mut self, name: &'a str, input: S) -> S
    where
        S: LogicalSource<'a> + IntoLogicalTarget<'a, REQUIRED> + 'a,
    {
        self.targets
            .push(Box::new(input.into_target(name, self.targets.len())));
        input
    }
}
