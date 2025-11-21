use std::{marker::PhantomData, ops::Deref};

use crate::dsl::{Identify, LogicalNode};

pub struct Node<'a, T: LogicalNode<'a> + ?Sized>(Box<T>, PhantomData<&'a T>);

impl<'a, T: LogicalNode<'a> + ?Sized> Identify for Node<'a, T> {
    fn identity(&self) -> u64 {
        self.0.identity()
    }
}

impl<'a, T: LogicalNode<'a> + Sized> LogicalNode<'a> for Node<'a, T> {
    fn name(&self) -> String {
        self.0.name()
    }

    fn outputs(&'a self) -> Vec<Box<dyn super::LogicalSource<'a> + 'a>> {
        self.0.outputs()
    }

    fn inputs<'b>(&'b self) -> Vec<&'b dyn super::LogicalTarget<'a>>
    where
        'a: 'b,
    {
        self.0.inputs()
    }

    fn into_compute(
        self,
    ) -> Result<Box<dyn crate::physical::ComputeNode>, super::IntoComputeNodeError> {
        self.0.into_compute()
    }
}

impl<'a, T: LogicalNode<'a> + Sized> Node<'a, T> {
    pub fn new(node: T) -> Node<'a, T> {
        Node(Box::new(node), PhantomData)
    }

    pub fn node(&'_ self) -> &'a dyn LogicalNode<'_> {
        self.0.as_ref()
    }
}

impl<'a, T: LogicalNode<'a>> Deref for Node<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
