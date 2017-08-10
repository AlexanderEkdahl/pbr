use energy::Energy;
use direction::Direction;
use rand::{Rng, ThreadRng};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Material {
    pub color: Energy, // Diffuse color for opaque surfaces, transmission coefficients for transparent surfaces
    pub fresnel: Energy, // Fresnel coefficients, used for fresnel reflectivity and computing the refractive index
    pub light: Energy, // Light emittance, used if this Material is a light source
    pub transmit: f64, // 0 = opaque, 1 = transparent, (0-1) = tinted thin surface
    pub gloss: f64, // Microsurface roughness (Material "polish")
    pub metal: f64, // The metallic range of electric (1) or dielectric (0), controls energy absorption

    init_absorbance: Energy, // Initd absorbance
    init_refract: f64, // Initd index of refraction
    init_fresnel: f64, // Initd average Fresnel value
}

impl Material {
    // TODO: Fix all of this..
    pub fn light(r: f64, g: f64, b: f64) -> Material {
        Material {
                color: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                fresnel: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                light: Energy { x: r, y: g, z: b },
                transmit: 0.0,
                gloss: 0.0,
                metal: 0.0,
                init_absorbance: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                init_refract: 0.0,
                init_fresnel: 0.0,
            }
            .init()
    }

    pub fn plastic(r: f64, g: f64, b: f64, gloss: f64) -> Material {
        Material {
                color: Energy { x: r, y: g, z: b },
                fresnel: Energy {
                    x: 0.04,
                    y: 0.04,
                    z: 0.04,
                },
                light: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                transmit: 0.0,
                gloss: gloss,
                metal: 0.0,
                init_absorbance: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                init_refract: 0.0,
                init_fresnel: 0.0,
            }
            .init()
    }

    pub fn lambert(r: f64, g: f64, b: f64) -> Material {
        Material {
                color: Energy { x: r, y: g, z: b },
                fresnel: Energy {
                    x: 0.02,
                    y: 0.02,
                    z: 0.02,
                },
                light: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                transmit: 0.0,
                gloss: 0.0,
                metal: 0.0,
                init_absorbance: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                init_refract: 0.0,
                init_fresnel: 0.0,
            }
            .init()
    }

    pub fn metal(r: f64, g: f64, b: f64, gloss: f64) -> Material {
        Material {
                color: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                fresnel: Energy { x: r, y: g, z: b },
                light: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                transmit: 0.0,
                gloss: gloss,
                metal: 1.0,
                init_absorbance: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                init_refract: 0.0,
                init_fresnel: 0.0,
            }
            .init()
    }

    pub fn glass(r: f64, g: f64, b: f64, gloss: f64) -> Material {
        Material {
                color: Energy { x: r, y: g, z: b },
                fresnel: Energy {
                    x: 0.042,
                    y: 0.042,
                    z: 0.042,
                },
                light: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                transmit: 1.0,
                gloss: gloss,
                metal: 0.0,
                init_absorbance: Energy {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                init_refract: 0.0,
                init_fresnel: 0.0,
            }
            .init()
    }

    fn init(mut self) -> Material {
        self.init_fresnel = self.fresnel.average().max(0.02);
        self.init_absorbance = Energy {
            x: 2.0 - (self.color.x * 100.0).log10(),
            y: 2.0 - (self.color.y * 100.0).log10(),
            z: 2.0 - (self.color.z * 100.0).log10(),
        };
        self.init_refract = (1.0 + self.init_fresnel.sqrt()) / (1.0 - self.init_fresnel.sqrt());

        self
    }

    pub fn bsdf(&self, norm: &Direction, inc: &Direction, dist: f64, rng: &mut ThreadRng) -> (bool, Direction, Energy) {
        if inc.enters(norm) {
            let reflect = schlick(norm, inc, self.init_fresnel, 0.0, 0.0);

            // https://doc.rust-lang.org/1.2.0/book/box-syntax-and-patterns.html
            // See above link for matcher with if
            if rng.gen_range(0.0, 1.0) < reflect {
                self.reflect(norm, inc, rng)
            } else if rng.gen_range(0.0, 1.0) < self.transmit {
                self.transmit(norm, inc, rng)
            } else if rng.gen_range(0.0, 1.0) < self.metal {
                self.absorb(inc)
            } else {
                self.diffuse(norm, inc, rng)
            }
        } else {
            self.exit(norm, inc, dist, rng)
        }
    }

    pub fn emit(&self, normal: &Direction, dir: &Direction) -> Energy {
        let cos = normal.dot(&dir.invert()).max(0.0);
        self.light.amplified(cos)
    }

    fn reflect(&self, norm: &Direction, inc: &Direction, rng: &mut ThreadRng) -> (bool, Direction, Energy) {
        let refl = inc.reflected(norm).cone(1.0 - self.gloss, rng);
        if refl.enters(norm) {
            self.diffuse(norm, inc, rng)
        } else {
            (true, refl, Energy{x: 1.0, y: 1.0, z: 1.0}.lerp(&self.fresnel, self.metal))
        }
    }

    fn transmit(&self, norm: &Direction, inc: &Direction, rng: &mut ThreadRng) -> (bool, Direction, Energy) {
        let (entered, refr) = inc.refracted(norm, 1.0, self.init_refract);

        if entered {
            let spread = refr.cone(1.0 - self.gloss, rng);

            if spread.enters(norm) {
                (true, spread, Energy{x: 1.0, y: 1.0, z: 1.0})
            } else {
                (true, refr, Energy{x: 1.0, y: 1.0, z: 1.0})
            }
        } else {
            self.diffuse(norm, inc, rng)
        }
    }

    // TODO: Replace these with beautiful pattern matches
    // And some kind of interaction trait? They are all very similar
    fn exit(&self, norm: &Direction, inc: &Direction, dist: f64, rng: &mut ThreadRng) -> (bool, Direction, Energy) {
        if self.transmit == 0.0 {
            return (true, Direction{x: inc.x, y: inc.y, z: inc.z}, Energy{x: 1.0, y: 1.0, z: 1.0})
        }

        if rng.gen_range(0.0, 1.0) >= schlick(norm, inc, 0.0, self.init_refract, 1.0) {
            let (exited, refr) = inc.refracted(&norm.invert(), self.init_refract, 1.0);
            if exited {
                let spread = refr.cone(1.0 - self.gloss, rng);

                if spread.enters(norm) {
                    return (true, spread, beers(dist, &self.init_absorbance));
                }

                return (true, refr, beers(dist, &self.init_absorbance));
            }
        }

        (true, inc.reflected(&norm.invert()), beers(dist, &self.init_absorbance))
    }

    // How can incoming not matter at all when diffusing?
    fn diffuse(&self, norm: &Direction, _: &Direction, rng: &mut ThreadRng) -> (bool, Direction, Energy) {
        (true, norm.random_hemi_cos(rng), self.color.amplified(1.0 / PI))
    }

    fn absorb(&self, inc: &Direction,) -> (bool, Direction, Energy) {
        (false, Direction{x: inc.x, y: inc.y, z: inc.z}, Energy{x: 0.0, y: 0.0, z: 0.0})
    }
}

fn schlick(incident: &Direction, normal: &Direction, mut r0: f64, n1: f64, n2: f64) -> f64 {
    let mut cos_x = -normal.dot(incident);

    if r0 == 0.0 {
        r0 = (n1 - n2) / (n1 + n2);
        r0 *= r0;

        if n1 > n2 {
            let n = n1 / n2;
            let sin_t2 = n * n * (1.0 - cos_x * cos_x);
            if sin_t2 > 1.0 {
                return 1.0;
            }
            cos_x = (1.0 - sin_t2).sqrt();
        }
    }
    let x = 1.0 - cos_x;

    r0 + (1.0 - r0) * x * x * x * x * x
}

fn beers(dist: f64, absorb: &Energy) -> Energy {
    let red = (-absorb.x * dist).exp();
    let green = (-absorb.y * dist).exp();
    let blue = (-absorb.z * dist).exp();
    Energy {
        x: red,
        y: green,
        z: blue,
    }
}
