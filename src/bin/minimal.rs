use std::fmt::Debug;
use std::f64::consts::PI;

trait Surface: Debug {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Scene<'a> {
    surfaces: &'a Vec<Box<Surface + 'a>>,
}

impl<'a> Scene<'a> {
    fn new(surfaces: &'a Vec<Box<Surface + 'a>>) -> Scene<'a> {
        Scene { surfaces: surfaces }
    }
}

#[derive(Debug)]
struct Sphere<'a> {
    radius: f64,
    material: &'a Material,
}

impl<'a> Surface for Sphere<'a> {
    fn area(&self) -> f64 {
        4.0 * PI * self.radius.powi(2)
    }
}

#[derive(Debug)]
struct Material {
    opacity: f64,
}

fn main() {
    let material = Material { opacity: 0.3 };
    let surfaces: Vec<Box<Surface>> = vec![Box::new(Sphere { radius: 3.5, material: &material })];
    let scene = Scene::new(&surfaces);

    println!("{:?}", scene);
}
