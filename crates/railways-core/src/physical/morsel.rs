use std::any::{type_name, type_name_of_val, Any};

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Error during value type casting (expected: {expected}, found: {found})")]
pub struct CastError {
    expected: String,
    found: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MorselContext {
    iter: bool,
}

pub struct Morsel {
    value: Box<dyn Any>,

    context: MorselContext,
}

impl Morsel {
    pub fn map<T, R>(self, f: impl Fn(T) -> R) -> Result<Self, CastError>
    where
        T: 'static,
        R: 'static,
    {
        let (inner, context) = self.inner::<T>()?;

        let mapped = f(inner);

        Ok(Self {
            value: Box::new(mapped),
            context: context,
        })
    }

    pub fn try_map<T, R, E>(
        self,
        f: impl Fn(T) -> Result<R, E>,
    ) -> Result<Result<Self, E>, CastError>
    where
        T: 'static,
        R: 'static,
    {
        let (inner, context) = self.inner::<T>()?;

        let mapped = f(inner);

        let mapped = mapped.map(|val| Self {
            value: Box::new(val),
            context: context,
        });

        Ok(mapped)
    }

    pub fn inner<T>(self) -> Result<(T, MorselContext), CastError>
    where
        T: 'static,
    {
        let mapped = self.value.downcast::<T>().map_err(|err| CastError {
            expected: type_name::<T>().to_string(),
            found: type_name_of_val(err.as_ref()).to_owned(),
        })?;

        Ok((*mapped, self.context))
    }
}
