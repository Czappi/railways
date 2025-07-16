use std::{marker::PhantomData, ops::Deref};

use crate::dsl::LogicalNode;

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
