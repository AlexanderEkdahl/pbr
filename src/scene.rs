use surface::Surface;
use std::f64::INFINITY;
use ray3::Ray3;
use constants::UP;
use energy::Energy;

#[derive(Debug)]
pub struct Scene<'a> {
    surfaces: &'a Vec<Box<Surface + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(surfaces: &'a Vec<Box<Surface + 'a>>) -> Scene<'a> {
        Scene {
            surfaces: surfaces,
        }
    }

    pub fn intersect(&self, ray: &Ray3) -> Option<(&Box<Surface + 'a>, f64)> {
        let dist = INFINITY;
        let mut result = None;

        for surface in self.surfaces.iter() {
            let (i, d) = surface.intersect(ray);

            if i && d < dist {
                result = Some((surface, dist))
            }
        }

        result
    }

    pub fn env(&self, ray: &Ray3) -> Energy {
        let vertical = ((ray.direction.dot(&UP) + 0.5) / 1.5).max(0.0);

        Energy {x: 0.0, y: 0.0, z: 0.0}.lerp(&Energy {x: 255.0, y: 255.0, z: 255.0}, vertical)
    }
}
