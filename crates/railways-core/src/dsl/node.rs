use std::{marker::PhantomData, ops::Deref};

use crate::dsl::{Identify, LogicalNode};

pub struct Node<'a, T: LogicalNode<'a> + ?Sized>(Box<T>, PhantomData<&'a T>);

impl<'a, T: LogicalNode<'a> + ?Sized> Identify for Node<'a, T> {
    fn identity(&self) -> u64 {
        self.0.identity()
    }
}

impl<'a, T: LogicalNode<'a> + ?Sized> LogicalNode<'a> for Node<'a, T> {
    fn name(&self) -> String {
        self.0.name()
    }

    fn sources(&'a self) -> Vec<Box<dyn super::LogicalSource<'a> + 'a>> {
        self.0.sources()
    }

    fn targets(&'a self) -> Vec<Box<dyn super::LogicalTarget<'a> + 'a>> {
        self.0.targets()
    }
}

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
