use direction::Direction;
use vector3::Vector3;
use ray3::Ray3;

const Y_AXIS: Vector3 = Direction {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

#[derive(Debug, PartialEq)]
pub struct Matrix4([[f64; 4]; 4]);

impl Matrix4 {
    fn new(a1: f64,
           a2: f64,
           a3: f64,
           a4: f64,
           b1: f64,
           b2: f64,
           b3: f64,
           b4: f64,
           c1: f64,
           c2: f64,
           c3: f64,
           c4: f64,
           d1: f64,
           d2: f64,
           d3: f64,
           d4: f64)
           -> Matrix4 {
        Matrix4([[a1, b1, c1, d1],
                 [a2, b2, c2, d2],
                 [a3, b3, c3, d3],
                 [a4, b4, c4, d4]])
    }

    pub fn identity() -> Matrix4 {
        Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0)
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4::new(1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0)
    }

    fn scale(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4::new(x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0)
    }

    fn rotation(v: &Vector3) -> Matrix4 {
        let a = v.len();
        let c = a.cos();
        let s = a.sin();
        let t = 1.0 - c;
        let n = v.unit();
        let (x, y, z) = (n.x, n.y, n.z);

        Matrix4::new(t * x * x + c,
                     t * x * y - z * s,
                     t * x * z + y * s,
                     0.0,
                     t * x * y + z * s,
                     t * y * y + c,
                     t * y * z - x * s,
                     0.0,
                     t * x * z - y * s,
                     t * y * z + x * s,
                     t * z * z + c,
                     0.0,
                     0.0,
                     0.0,
                     0.0,
                     1.0)
    }

    pub fn look_matrix(o: &Vector3, to: &Vector3) -> Matrix4 {
        let f = (o - to).unit();
        let r = Y_AXIS.cross(&f);
        let u = f.cross(&r);
        let orient = Matrix4::new(r.x, u.x, f.x, 0.0, r.y, u.y, f.y, 0.0, r.z, u.z, f.z, 0.0, 0.0, 0.0, 0.0, 1.0);

        Matrix4::translation(o.x, o.y, o.z).mult(&orient)
    }

    pub fn chain_translation(&self, x: f64, y: f64, z: f64) -> Matrix4 {
        self.mult(&Matrix4::translation(x, y, z))
    }

    pub fn chain_scale(&self, x: f64, y: f64, z: f64) -> Matrix4 {
        self.mult(&Matrix4::scale(x, y, z))
    }

    pub fn chain_rotation(&self, v: &Vector3) -> Matrix4 {
        self.mult(&Matrix4::rotation(v))
    }

    pub fn mult(&self, other: &Matrix4) -> Matrix4 {
        let mut m = Matrix4::identity();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    m.0[j][i] += self.0[k][i] * other.0[j][k];
                }
            }
        }

        m
    }

    pub fn mult_point(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: v.x * self.0[0][0] + v.y * self.0[1][0] + v.z * self.0[2][0] + self.0[3][0],
            y: v.x * self.0[0][1] + v.y * self.0[1][1] + v.z * self.0[2][1] + self.0[3][1],
            z: v.x * self.0[0][2] + v.y * self.0[1][2] + v.z * self.0[2][2] + self.0[3][2],
        }
    }

    pub fn mult_dist(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: v.x * self.0[0][0] + v.y * self.0[1][0] + v.z * self.0[2][0],
            y: v.x * self.0[0][1] + v.y * self.0[1][1] + v.z * self.0[2][1],
            z: v.x * self.0[0][2] + v.y * self.0[1][2] + v.z * self.0[2][2],
        }
    }

    pub fn mult_dir(&self, v: &Vector3) -> Direction {
        self.mult_dist(v).unit()
    }

    pub fn mult_ray(&self, r: &Ray3) -> Ray3 {
        Ray3 {
            origin: self.mult_point(&r.origin),
            direction: self.mult_dir(&r.direction),
        }
    }

    pub fn inverse(&self) -> Matrix4 {
        let mut i = Matrix4::identity();

        i.0[0][0] = self.0[1][1] * self.0[2][2] * self.0[3][3] - self.0[1][1] * self.0[2][3] * self.0[3][2] -
                    self.0[2][1] * self.0[1][2] * self.0[3][3] + self.0[2][1] * self.0[1][3] * self.0[3][2] +
                    self.0[3][1] * self.0[1][2] * self.0[2][3] - self.0[3][1] * self.0[1][3] * self.0[2][2];
        i.0[1][0] = self.0[1][0] * self.0[2][3] * self.0[3][2] - self.0[1][0] * self.0[2][2] * self.0[3][3] +
                    self.0[2][0] * self.0[1][2] * self.0[3][3] - self.0[2][0] * self.0[1][3] * self.0[3][2] -
                    self.0[3][0] * self.0[1][2] * self.0[2][3] + self.0[3][0] * self.0[1][3] * self.0[2][2];
        i.0[2][0] = self.0[1][0] * self.0[2][1] * self.0[3][3] - self.0[1][0] * self.0[2][3] * self.0[3][1] -
                    self.0[2][0] * self.0[1][1] * self.0[3][3] + self.0[2][0] * self.0[1][3] * self.0[3][1] +
                    self.0[3][0] * self.0[1][1] * self.0[2][3] - self.0[3][0] * self.0[1][3] * self.0[2][1];
        i.0[3][0] = self.0[1][0] * self.0[2][2] * self.0[3][1] - self.0[1][0] * self.0[2][1] * self.0[3][2] +
                    self.0[2][0] * self.0[1][1] * self.0[3][2] - self.0[2][0] * self.0[1][2] * self.0[3][1] -
                    self.0[3][0] * self.0[1][1] * self.0[2][2] + self.0[3][0] * self.0[1][2] * self.0[2][1];
        i.0[0][1] = self.0[0][1] * self.0[2][3] * self.0[3][2] - self.0[0][1] * self.0[2][2] * self.0[3][3] +
                    self.0[2][1] * self.0[0][2] * self.0[3][3] - self.0[2][1] * self.0[0][3] * self.0[3][2] -
                    self.0[3][1] * self.0[0][2] * self.0[2][3] + self.0[3][1] * self.0[0][3] * self.0[2][2];
        i.0[1][1] = self.0[0][0] * self.0[2][2] * self.0[3][3] - self.0[0][0] * self.0[2][3] * self.0[3][2] -
                    self.0[2][0] * self.0[0][2] * self.0[3][3] + self.0[2][0] * self.0[0][3] * self.0[3][2] +
                    self.0[3][0] * self.0[0][2] * self.0[2][3] - self.0[3][0] * self.0[0][3] * self.0[2][2];
        i.0[2][1] = self.0[0][0] * self.0[2][3] * self.0[3][1] - self.0[0][0] * self.0[2][1] * self.0[3][3] +
                    self.0[2][0] * self.0[0][1] * self.0[3][3] - self.0[2][0] * self.0[0][3] * self.0[3][1] -
                    self.0[3][0] * self.0[0][1] * self.0[2][3] + self.0[3][0] * self.0[0][3] * self.0[2][1];
        i.0[3][1] = self.0[0][0] * self.0[2][1] * self.0[3][2] - self.0[0][0] * self.0[2][2] * self.0[3][1] -
                    self.0[2][0] * self.0[0][1] * self.0[3][2] + self.0[2][0] * self.0[0][2] * self.0[3][1] +
                    self.0[3][0] * self.0[0][1] * self.0[2][2] - self.0[3][0] * self.0[0][2] * self.0[2][1];
        i.0[0][2] = self.0[0][1] * self.0[1][2] * self.0[3][3] - self.0[0][1] * self.0[1][3] * self.0[3][2] -
                    self.0[1][1] * self.0[0][2] * self.0[3][3] + self.0[1][1] * self.0[0][3] * self.0[3][2] +
                    self.0[3][1] * self.0[0][2] * self.0[1][3] - self.0[3][1] * self.0[0][3] * self.0[1][2];
        i.0[1][2] = self.0[0][0] * self.0[1][3] * self.0[3][2] - self.0[0][0] * self.0[1][2] * self.0[3][3] +
                    self.0[1][0] * self.0[0][2] * self.0[3][3] - self.0[1][0] * self.0[0][3] * self.0[3][2] -
                    self.0[3][0] * self.0[0][2] * self.0[1][3] + self.0[3][0] * self.0[0][3] * self.0[1][2];
        i.0[2][2] = self.0[0][0] * self.0[1][1] * self.0[3][3] - self.0[0][0] * self.0[1][3] * self.0[3][1] -
                    self.0[1][0] * self.0[0][1] * self.0[3][3] + self.0[1][0] * self.0[0][3] * self.0[3][1] +
                    self.0[3][0] * self.0[0][1] * self.0[1][3] - self.0[3][0] * self.0[0][3] * self.0[1][1];
        i.0[3][2] = self.0[0][0] * self.0[1][2] * self.0[3][1] - self.0[0][0] * self.0[1][1] * self.0[3][2] +
                    self.0[1][0] * self.0[0][1] * self.0[3][2] - self.0[1][0] * self.0[0][2] * self.0[3][1] -
                    self.0[3][0] * self.0[0][1] * self.0[1][2] + self.0[3][0] * self.0[0][2] * self.0[1][1];
        i.0[0][3] = self.0[0][1] * self.0[1][3] * self.0[2][2] - self.0[0][1] * self.0[1][2] * self.0[2][3] +
                    self.0[1][1] * self.0[0][2] * self.0[2][3] - self.0[1][1] * self.0[0][3] * self.0[2][2] -
                    self.0[2][1] * self.0[0][2] * self.0[1][3] + self.0[2][1] * self.0[0][3] * self.0[1][2];
        i.0[1][3] = self.0[0][0] * self.0[1][2] * self.0[2][3] - self.0[0][0] * self.0[1][3] * self.0[2][2] -
                    self.0[1][0] * self.0[0][2] * self.0[2][3] + self.0[1][0] * self.0[0][3] * self.0[2][2] +
                    self.0[2][0] * self.0[0][2] * self.0[1][3] - self.0[2][0] * self.0[0][3] * self.0[1][2];
        i.0[2][3] = self.0[0][0] * self.0[1][3] * self.0[2][1] - self.0[0][0] * self.0[1][1] * self.0[2][3] +
                    self.0[1][0] * self.0[0][1] * self.0[2][3] - self.0[1][0] * self.0[0][3] * self.0[2][1] -
                    self.0[2][0] * self.0[0][1] * self.0[1][3] + self.0[2][0] * self.0[0][3] * self.0[1][1];
        i.0[3][3] = self.0[0][0] * self.0[1][1] * self.0[2][2] - self.0[0][0] * self.0[1][2] * self.0[2][1] -
                    self.0[1][0] * self.0[0][1] * self.0[2][2] + self.0[1][0] * self.0[0][2] * self.0[2][1] +
                    self.0[2][0] * self.0[0][1] * self.0[1][2] - self.0[2][0] * self.0[0][2] * self.0[1][1];

        let det = 1.0 / (self.0[0][0] * i.0[0][0] + self.0[0][1] * i.0[1][0] + self.0[0][2] * i.0[2][0] + self.0[0][3] * i.0[3][0]);

        for j in 0..4 {
            for k in 0..4 {
                i.0[j][k] *= det;
            }
        }

        i
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix4;

    #[test]
    fn matrix4_inverse() {
        let a = Matrix4::identity();
        let b = a.inverse();

        assert_eq!(a, b);
    }
}
