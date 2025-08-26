use std::{
    any::{type_name, TypeId},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::dsl::{Identify, LogicalNode, LogicalSource, LogicalTarget};

/// [Value] indicate that a `T` typed value will be received.
///
/// This struct is not holding the value only **indicate** the
/// connection to the source [Node] that going to give the said value.
pub struct Value<'a, T> {
    source: &'a dyn LogicalNode<'a>,
    index: usize,
    p: PhantomData<T>,
}

impl<'a, T> Hash for Value<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"Value");
        state.write_u64(self.source.identity());
        Hash::hash(&self.index, state);
        Hash::hash(&self.p, state);
    }
}

impl<'a, T: Eq> Eq for Value<'a, T> {}

impl<'a, T: PartialEq> PartialEq for Value<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.source.identity() == other.source.identity()
            && self.index == other.index
            && self.p == other.p
    }
}

impl<'a, T: 'static> LogicalSource<'a> for Value<'a, T> {
    fn source(&'a self) -> &'a dyn LogicalNode<'a> {
        self.source
    }

    fn index(&self) -> usize {
        self.index
    }

    fn info(&self) -> super::PinInformation {
        super::PinInformation::builder()
            .name("Value")
            .ty(TypeId::of::<T>())
            .ty_name(type_name::<T>())
            .build()
    }
}

impl<'a, T> Value<'a, T> {
    pub fn new(source: &'a dyn LogicalNode<'a>, index: usize) -> Self {
        Self {
            source,
            index,
            p: PhantomData,
        }
    }
}

pub struct ValueTarget<'a, T, const REQUIRED: bool> {
    source: Option<&'a Value<'a, T>>,
    index: usize,
}

impl<'a, T, const REQUIRED: bool> Hash for ValueTarget<'a, T, REQUIRED> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"ValueTarget");
        state.write_u64(self.source.identity());
        Hash::hash(&self.index, state);
    }
}

impl<'a, T: 'static, const REQUIRED: bool> LogicalTarget<'a> for ValueTarget<'a, T, REQUIRED> {
    fn source(&'a self) -> Option<&'a dyn LogicalSource<'a>> {
        self.source.map(|s| s as &'a dyn LogicalSource<'a>)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn is_required(&self) -> bool {
        REQUIRED
    }

    fn info(&self) -> super::PinInformation {
        super::PinInformation::builder()
            .name("ValueTarget")
            .ty(TypeId::of::<T>())
            .ty_name(type_name::<T>())
            .required(REQUIRED)
            .build()
    }
}

impl<'a, T: 'static, const REQUIRED: bool> ValueTarget<'a, T, REQUIRED> {
    pub fn new(source: impl Into<Option<&'a Value<'a, T>>>, index: usize) -> Self {
        Self {
            source: source.into(),
            index,
        }
    }
}
