use cgmath::{Vector3, Point3, BaseNum};

/// A generic ray starting at `origin` and extending infinitely in
/// `direction`.
#[derive(Copy, Clone, PartialEq)]
pub struct Ray<S> {
    pub origin: Point3<S>,
    pub direction: Vector3<S>
}

impl<S: BaseNum> Ray<S> {
    pub fn new(origin: Point3<S>, direction: Vector3<S>) -> Ray<S> {
        Ray {
        	origin: origin,
        	direction: direction
        }
    }
}
