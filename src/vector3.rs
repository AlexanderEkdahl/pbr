use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn average(&self) -> f64 {
        (self.x + self.y + self.z) / 3.0
    }

    pub fn max(&self) -> f64 {
        self.x.max(self.y.max(self.z))
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn lerp(&self, other: &Vector3, n: f64) -> Vector3 {
        let m = 1.0 - n;
        Vector3 {
            x: self.x * m + other.x * n,
            y: self.y * m + other.y * n,
            z: self.z * m + other.z * n,
        }
    }

    pub fn abs(&self) -> Vector3 {
        Vector3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    // Moved from direction
    pub fn unit(&self) -> Vector3 {
        let d = self.len();
        Vector3 {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector3 {{ {}, {}, {} }}", self.x, self.y, self.z)
    }
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &'b Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &'b Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a, 'b> Mul<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &'b Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<'a> Mul<f64> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector3;

    #[test]
    fn vector3_add() {
        let v1 = Vector3 {
            x: 0.0,
            y: 1.2,
            z: 2.8,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: -4.2,
            z: 1.0,
        };

        assert_eq!(Vector3 {
                       x: 4.0,
                       y: -3.0,
                       z: 3.8,
                   },
                   &v1 + &v2);
    }

    #[test]
    fn vector3_mul() {
        let v1 = Vector3 {
            x: 0.0,
            y: 2.0,
            z: 2.8,
        };
        let v2 = Vector3 {
            x: 2.0,
            y: 1.5,
            z: 1.0,
        };

        assert_eq!(Vector3 {
                       x: 0.0,
                       y: 3.0,
                       z: 2.8,
                   },
                   &v1 * &v2);
    }

    #[test]
    fn vector3_scale() {
        let v1 = Vector3 {
            x: 0.0,
            y: 2.0,
            z: 2.8,
        };

        assert_eq!(Vector3 {
                       x: 0.0,
                       y: 10.0,
                       z: 14.0,
                   },
                   &v1 * 5.0);
    }

    #[test]
    fn vector3_abs() {
        let v = Vector3 {
            x: -0.0,
            y: -2.0,
            z: 2.8,
        };

        assert_eq!(Vector3 {
                       x: 0.0,
                       y: 2.0,
                       z: 2.8,
                   },
                   v.abs());
    }
}
