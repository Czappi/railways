use std::{
    any::{type_name, TypeId},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::dsl::{Identify, IntoLogicalTarget, LogicalNode, LogicalSource, LogicalTarget};

/// [List] indicate that a list of `T` typed values will be received.
///
/// This struct is not holding the list of values only **indicate** the
/// connection to the source [Node] that going to give the said value.
#[derive(Copy)]
pub struct List<'a, T> {
    source: &'a dyn LogicalNode<'a>,
    name: &'a str,
    index: usize,
    p: PhantomData<T>,
}

impl<'a, T: 'static, const REQUIRED: bool> IntoLogicalTarget<'a, REQUIRED> for List<'a, T> {
    type Target = ListTarget<'a, T, REQUIRED>;

    fn into_target(&self, name: &'a str, index: usize) -> Self::Target {
        ListTarget::new(self.clone(), name, index)
    }
}

impl<'a, T> Clone for List<'a, T> {
    fn clone(&self) -> Self {
        Self {
            source: self.source,
            name: self.name,
            index: self.index.clone(),
            p: self.p.clone(),
        }
    }
}

impl<'a, T: 'static> LogicalSource<'a> for List<'a, T> {
    fn node(&'a self) -> &'a dyn LogicalNode<'a> {
        self.source
    }

    fn index(&self) -> usize {
        self.index
    }

    fn info(&self) -> super::PinInformation {
        super::PinInformation::builder()
            .name(self.name)
            .ty(TypeId::of::<T>())
            .ty_name(type_name::<T>())
            .build()
    }

    fn boxed_clone(&self) -> Box<dyn LogicalSource<'a> + 'a> {
        Box::new(self.clone()) as Box<dyn LogicalSource<'a> + 'a>
    }
}

impl<'a, T: PartialEq> PartialEq for List<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.source.identity() == other.source.identity()
            && self.index == other.index
            && self.p == other.p
    }
}

impl<'a, T: Eq> Eq for List<'a, T> {}

impl<'a, T> Hash for List<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(b"List");
        state.write(self.name.as_bytes());
        state.write_usize(self.index);
        state.write_u64(self.source.identity());
        Hash::hash(&self.p, state);
    }
}

#[derive(Copy)]
pub struct ListTarget<'a, T, const REQUIRED: bool> {
    source: Option<List<'a, T>>,
    name: &'a str,
    index: usize,
}

impl<'a, T, const REQUIRED: bool> Clone for ListTarget<'a, T, REQUIRED> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            name: self.name,
            index: self.index.clone(),
        }
    }
}

impl<'a, T, const REQUIRED: bool> Hash for ListTarget<'a, T, REQUIRED> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"ListTarget");
        state.write(self.name.as_bytes());
        state.write_usize(self.index);
        state.write_u64(self.source.identity());
    }
}

impl<'a, T: 'static, const REQUIRED: bool> LogicalTarget<'a> for ListTarget<'a, T, REQUIRED> {
    fn source<'b>(&'b self) -> Option<&'b dyn LogicalSource<'a>>
    where
        'a: 'b,
    {
        self.source.as_ref().map(|s| s as &'b dyn LogicalSource<'a>)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn is_required(&self) -> bool {
        REQUIRED
    }

    fn info(&self) -> super::PinInformation {
        super::PinInformation::builder()
            .name(self.name)
            .ty(TypeId::of::<T>())
            .ty_name(type_name::<T>())
            .required(REQUIRED)
            .build()
    }

    fn boxed_clone(&self) -> Box<dyn LogicalTarget<'a> + 'a> {
        Box::new(self.clone())
    }
}

impl<'a, T: 'static, const REQUIRED: bool> ListTarget<'a, T, REQUIRED> {
    pub fn new(source: impl Into<Option<List<'a, T>>>, name: &'a str, index: usize) -> Self {
        Self {
            source: source.into(),
            name,
            index,
        }
    }
}
