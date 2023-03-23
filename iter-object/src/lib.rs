pub use iter_object_derive::*;

pub trait IterObject<T> {
    fn to_params(self) -> Vec<T>;
}
