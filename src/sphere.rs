use material::Material;
use matrix4::Matrix4;
use vector3::Vector3;
use surface::Surface;
use ray3::Ray3;
use constants::BIAS;

#[derive(Debug)]
pub struct Sphere<'a> {
    material: &'a Material,
    pos: Matrix4,
}

impl<'a> Sphere<'a> {
    pub fn new(m: &'a Material) -> Sphere<'a> {
        Sphere {
            material: m,
            pos: Matrix4::identity(),
        }
    }
}

impl<'a> Surface for Sphere<'a> {
    fn intersect(&self, r: &Ray3) -> (bool, f64) {
        let i = self.pos.inverse();
        let r = i.mult_ray(r);
        let op = r.origin.invert();
        let b = op.dot(&r.direction);
        let det = b * b - op.dot(&op) + 0.25;
        if det < 0.0 {
            return (false, 0.0);
        }
        let root = det.sqrt();
        let t1 = b - root;
        if t1 > 0.0 {
            let dist = self.pos.mult_dist(&(&r.direction * t1)).len();

            if dist > BIAS {
                return (true, dist);
            }
        }

        let t2 = b + root;
        if t2 > 0.0 {
            let dist = self.pos.mult_dist(&(&r.direction * t2)).len();

            if dist > BIAS {
                return (true, dist);
            }
        }

        (false, 0.0)
    }

    fn at(&self, v: &Vector3) -> (Vector3, &Material) {
        let i = self.pos.inverse();
        let p = i.mult_point(v);

        (self.pos.mult_dir(&p.unit()), self.material)
    }
}
