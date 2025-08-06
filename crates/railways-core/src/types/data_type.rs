use thiserror::Error;

#[derive(Error, Debug)]
#[error("Error during upcasting a value")]
pub struct UpcastError;

pub type AnyBox = Box<dyn std::any::Any>;

/// Data type marker trait
pub trait DataType: Sized + 'static {
    fn upcast(value: AnyBox) -> Result<Self, UpcastError> {
        value
            .downcast::<Self>()
            .map_err(|_| UpcastError)
            .map(|val| *val)
    }

    fn downcast(value: Self) -> AnyBox {
        Box::new(value)
    }

    fn io_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

impl<T: Sized + 'static> DataType for T {}
