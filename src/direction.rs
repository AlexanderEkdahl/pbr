use vector3::Vector3;
use std::f64::consts::PI;
use rand::{Rng, ThreadRng};

pub type Direction = Vector3;

impl Direction {
    pub fn invert(&self) -> Direction {
        Direction {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn enters(&self, normal: &Direction) -> bool {
        normal.dot(&self) < 0.0
    }

    pub fn cos(&self, b: &Direction) -> f64 {
        self.dot(b)
    }

    pub fn refracted(&self, normal: &Direction, index_a: f64, index_b: f64) -> (bool, Direction) {
        let ratio = index_a / index_b;
        let cos = normal.cos(self);
        let k = 1.0 - ratio * ratio * (1.0 - cos * cos);

        if k < 0.0 {
            (false, Direction{x: self.x, y: self.y, z: self.z})
        } else {
            let offset = normal * (ratio * cos + k.sqrt());
            (true, (&(self * ratio) - &offset).unit())
        }
    }

    pub fn reflected(&self, normal: &Direction) -> Direction {
        let cos = normal.cos(self);
        (self - &(&(normal * 2.0) * cos)).unit()
    }

    pub fn cone(&self, size: f64, rng: &mut ThreadRng) -> Direction {
        let u = rng.gen_range(0.0, 1.0);
        let v = rng.gen_range(0.0, 1.0);
        let theta = size * 0.5 * PI * (1.0 - (2.0 * (u as f64).acos() / PI));
        let m1 = theta.sin();
        let m2 = theta.cos();
        let a2 = v * 2.0 * PI;
        let q = Direction::random(rng);
        let s = self.cross(&q);
        let t = self.cross(&s);
        let mut d = Vector3{x: 0.0, y: 0.0, z: 0.0 };
        d = &d + &(&(&s * m1) * a2.cos());
        d = &d + &(&(&t * m1) * a2.cos());
        d = &d + &(self * m2);
        d.unit()
    }

    pub fn random(rng: &mut ThreadRng) -> Direction {
        Direction::angle_direction(rng.gen_range::<f64>(0.0, 2.0 * PI), rng.gen_range::<f64>(-1.0, 1.0).asin())
    }

    pub fn angle_direction(theta: f64, phi: f64) -> Direction {
        Direction{
            x: theta.cos() * phi.cos(),
            y: phi.sin(),
            z: theta.sin() * phi.cos(),
        }
    }

    pub fn random_hemi_cos(&self, rng: &mut ThreadRng) -> Direction {
        let u = rng.gen_range::<f64>(0.0, 1.0);
        let r = u.sqrt();
        let theta = rng.gen_range::<f64>(0.0, 2.0 * PI);
        let s = self.cross(&Direction::random(rng));
        let t = self.cross(&s);
        let mut d = Vector3{x: 0.0, y: 0.0, z: 0.0 };
        d = &d + &(&s * (r * theta.cos()));
        d = &d + &(&t * (r * theta.sin()));
        d = &d + &(self * (1.0 - u).sqrt());
        d
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn direction_unit() {
        let d = Direction {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq!(Direction {
                       x: 1.0,
                       y: 0.0,
                       z: 0.0,
                   },
                   d.unit());
    }
}
