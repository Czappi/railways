/// A marker type for `Option<T>` to indicate that a value is optional.
/// This indicates that the value is optional and **NOT** a source which returns an `Option<T>`.
type Optional<T> = Option<T>;
