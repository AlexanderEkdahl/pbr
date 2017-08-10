// extern crate pbr;
// extern crate rand;

// use pbr::scene::Scene;
// use pbr::sphere::Sphere;
// use pbr::material::Material;
// use pbr::surface::Surface;
// use pbr::renderer::Renderer;
// use pbr::sampler::{Sampler, SamplerConfiguration};
// use pbr::camera::Camera;
// use pbr::ray3::Ray3;
// use pbr::vector3::Vector3;
// use pbr::direction::Direction;

// fn main() {
//     let mut rng = rand::thread_rng();
//     let material = Material::plastic(1.0, 0.3, 0.4, 0.9);
//     let surfaces: Vec<Box<Surface>> = vec![Box::new(Sphere::new(&material))];
//     let scene = Scene::new(&surfaces);
//     let camera = Camera::new35mm(20, 10);
//     let mut sampler = Sampler::new(&camera, &scene, SamplerConfiguration {
//         max_bounces: 10,
//         adapt: 4,
//     });

//     let r = Ray3 {
//         origin: Vector3 {
//             x: 0.0,
//             y: 0.0,
//             z: 3.0,
//         },
//         direction: Direction {
//             x: 0.0,
//             y: 0.0,
//             z: 1.0,
//         }
//     };
//     let energy = scene.env(&r);
// 	println!("{:?}", energy);

//     // for x in 0..camera.width {
//     //     for y in 0..camera.height {
//     //         sampler.sample_pixel(x, y, &mut rng, 1);
//     //     }
//     // }

//     let r = Renderer::new(&sampler);
//     r.png()
// }
