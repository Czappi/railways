/// Event which goes through the pipeline
///
/// It contains the event state, route
pub struct Parcel<T, E> {
    route: Vec<usize>,
    inner: Result<T, E>,
}
