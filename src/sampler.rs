use camera::Camera;
use energy::Energy;
use rand::ThreadRng;
use scene::Scene;
use ray3::Ray3;
use std::fmt;
use sample::Sample;

#[derive(Debug)]
pub struct SamplerConfiguration {
    pub max_bounces: usize,
    pub adapt: usize,
}

pub struct Sampler<'a> {
    config: SamplerConfiguration,
    pub samples: Vec<Vec<Sample>>,
    pub cam: &'a Camera,
    scene: &'a Scene<'a>,
}

impl<'a> Sampler<'a> {
    pub fn new(camera: &'a Camera, scene: &'a Scene, config: SamplerConfiguration) -> Sampler<'a> {
        Sampler {
            config: config,
            samples: vec![vec![Sample { red: 0.0, green: 0.0, blue: 0.0, count: 0 }; camera.height]; camera.width],
            cam: camera,
            scene: scene,
        }
    }

    pub fn sample_pixel(&mut self, x: usize, y: usize, rng: &mut ThreadRng, samples: usize) {
        // println!("self.config.width: {}", self.config.width);
        // println!("self.config.height: {}", self.config.height);
        // println!("x: {} y: {}", x as f64 / self.config.width as f64, y as f64 / self.config.height as f64);

        for _ in 0..samples {
            let sample = self.trace(x as f64 / self.cam.width as f64, y as f64 / self.cam.height as f64, rng);
            // println!("{:?}", sample);
            self.samples[x][y] = Sample {
                red: self.samples[x][y].red + sample.x,
                green: self.samples[x][y].green + sample.y,
                blue: self.samples[x][y].blue + sample.z,
                count: self.samples[x][y].count + 1,
            };
        }
        // println!("{:?}", self.samples[x][y]);
    }

    pub fn trace(&self, x: f64, y: f64, rng: &mut ThreadRng) -> Energy {
        let mut ray = self.cam.ray(x, y, rng);
        let mut energy = Energy{x: 0.0, y: 0.0, z: 0.0};
        let mut signal = Energy{x: 1.0, y: 1.0, z: 1.0};

        for _ in 0..self.config.max_bounces {
            if let Some((surface, dist)) = self.scene.intersect(&ray) {
                let point = ray.moved(dist);
                let (normal, mat) = surface.at(&point);
                energy = energy.merged(&mat.emit(&normal, &ray.direction), &signal);

                if let Some(newsignal) = signal.random_gain(rng) {
                    signal = newsignal;
                } else {
                    return energy;
                }

                if let (true, direction, strength) = mat.bsdf(&normal, &ray.direction, dist, rng) {
                    signal = &signal * &strength;
                    ray = Ray3 {
                        origin: point,
                        direction: direction,
                    }
                } else {
                    return energy;
                }
            } else {
                return energy.merged(&self.scene.env(&ray), &signal);
            }
        }

        energy
    }
}

impl<'a> fmt::Debug for Sampler<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sampler {{ config: {:?}, cam: {:?}, scene: {:?} }}", self.config, self.cam, self.scene)
    }
}
