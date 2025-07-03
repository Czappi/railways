use std::{marker::PhantomData, ops::Deref, rc::Rc};

use dyn_any::DynAny;

pub trait LogicalNode<'a> {
    fn name(&'a self) -> String;

    fn input(&self, index: usize) -> Result<Link<'a, dyn std::any::Any>, String>;
}

pub struct Node<'a, T: LogicalNode<'a> + ?Sized>(Box<T>, PhantomData<&'a T>);

impl<'a, T: LogicalNode<'a>> Deref for Node<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub struct U64Node(u64);

pub fn u64(value: u64) -> Link<'static, u64> {
    U64Node::new(value)
}

impl U64Node {
    pub fn new(value: u64) -> Link<'static, u64> {
        Link(&Source {
            node: &Node(Box::new(Self(value)), PhantomData),
            index: 0,
            p: PhantomData,
        })
    }
}

impl<'a> LogicalNode<'a> for U64Node {
    fn name(&'a self) -> String {
        self.0.to_string()
    }

    fn input<T>(index: usize) -> Result<Link<'a, T>, String> {
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u64>() {}
    }
}

pub struct TNode<'a> {
    id: Link<'a, u64>,
}

pub fn node<'a>(id: Link<'a, u64>) -> Node<TNode<'a>> {
    Box::new(Node { id })
}

impl<'a> LogicalNode<'a> for Node<'a> {
    fn name(&'a self) -> String {
        "Node".to_owned()
    }
}

pub struct Source<'a, T> {
    pub node: &'a Node<'a, dyn LogicalNode<'a>>,
    index: u64,
    p: PhantomData<T>,
}

pub struct Link<'a, T>(&'a Source<'a, T>);

#[test]
fn test() {
    let u = u64(69);

    let node = node(u);

    println!("{}", node.id.0.node.);
}
