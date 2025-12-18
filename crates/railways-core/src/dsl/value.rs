use std::{
    any::{type_name, Any, TypeId},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::dsl::{
    Identify, IntoLogicalTarget, LogicalNode, LogicalSource, LogicalTarget, PinConnectionError,
};

/// [Value] indicate that a `T` typed value will be received.
///
/// This struct is not holding the value only **indicate** the
/// connection to the source [Node] that going to give the said value.
#[derive(Copy)]
pub struct Value<'a, T> {
    source: &'a dyn LogicalNode<'a>,
    name: &'a str,
    index: usize,
    p: PhantomData<T>,
}

impl<'a, T: 'static, const REQUIRED: bool> IntoLogicalTarget<'a, REQUIRED> for Value<'a, T> {
    type Target = ValueTarget<'a, T, REQUIRED>;

    fn into_target(&self, name: &'a str, index: usize) -> Self::Target {
        ValueTarget::new(self.clone(), name, index)
    }
}

impl<'a, T> Clone for Value<'a, T> {
    fn clone(&self) -> Self {
        Self {
            source: self.source,
            name: self.name,
            index: self.index.clone(),
            p: self.p.clone(),
        }
    }
}

impl<'a, T> Hash for Value<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"Value");
        state.write(self.name.as_bytes());
        state.write_usize(self.index);
        state.write_u64(self.source.identity());
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

impl<'a, T> Value<'a, T> {
    pub fn new(source: &'a dyn LogicalNode<'a>, name: &'a str, index: usize) -> Self {
        Self {
            source,
            name,
            index,
            p: PhantomData,
        }
    }
}

#[derive(Copy, PartialEq, Eq)]
pub struct ValueTarget<'a, T, const REQUIRED: bool> {
    source: Option<Value<'a, T>>,
    name: &'a str,
    index: usize,
}

impl<'a, T, const REQUIRED: bool> Clone for ValueTarget<'a, T, REQUIRED> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            name: self.name,
            index: self.index.clone(),
        }
    }
}

impl<'a, T, const REQUIRED: bool> Hash for ValueTarget<'a, T, REQUIRED> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(b"ValueTarget");
        state.write(self.name.as_bytes());
        state.write_usize(self.index);
        state.write_u64(self.source.identity());
    }
}

impl<'a, T: 'static, const REQUIRED: bool> LogicalTarget<'a> for ValueTarget<'a, T, REQUIRED> {
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

    fn set_source(
        &'a mut self,
        source: &'a dyn LogicalSource<'a>,
    ) -> Result<(), PinConnectionError> {
        if let Some(source) = downcast_logical_target::<Value<'a, T>>(source) {
            self.source = Some(source.clone());
            Ok(())
        } else {
            Err(PinConnectionError::TypeMismatch {
                source_id: source.identity(),
                target_id: self.identity(),
                source_type: source.info().ty_name.unwrap_or_default(),
                target_type: self.info().ty_name.unwrap_or_default(),
            })
        }
    }
}

impl<'a, T: 'static, const REQUIRED: bool> ValueTarget<'a, T, REQUIRED> {
    pub fn new(source: impl Into<Option<Value<'a, T>>>, name: &'a str, index: usize) -> Self {
        Self {
            source: source.into(),
            name: name,
            index,
        }
    }
}
