#[macro_export]
macro_rules! impl_downcast_ref {
    ($name:stmt, $t:ty, $type_id:expr) => {
        place_macro::place! {
            fn __identifier__(downcast_ $name) <'a, T: $t + 'a>(
                reference: &'a dyn $t,
                ) -> Option<&'a T> {
                if reference. $type_id == TypeId::of::<Box<T>>() {
                    // SAFETY: caller guarantees that T is the correct type
                    unsafe { Some(&*(reference as *const dyn $t as *const T)) }
                } else {
                    None
                }
            }
        }
    };
}

pub use impl_downcast_ref;
