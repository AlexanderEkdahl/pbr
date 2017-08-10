use ray3::Ray3;
use vector3::Vector3;
use material::Material;
use std::fmt::Debug;

pub trait Surface: Debug {
    fn intersect(&self, r: &Ray3) -> (bool, f64);
    fn at(&self, v: &Vector3) -> (Vector3, &Material);
}
