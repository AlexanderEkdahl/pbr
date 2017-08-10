use vector3::Vector3;
use rand::{Rng, ThreadRng};

pub type Energy = Vector3;

impl Energy {
    pub fn merged(&self, b: &Energy, signal: &Energy) -> Energy {
        self + &(b * signal)
    }

    pub fn amplified(&self, n: f64) -> Energy {
        self * n
    }

    pub fn random_gain(&self, rng: &mut ThreadRng) -> Option<Energy> {
        let max = self.max();

        if rng.gen_range(0.0, 1.0) > max {
            None
        } else {
            Some(self.amplified(1.0 / max))
        }
    }

    pub fn strength(&self, b: &Energy) -> Energy {
        self * b
    }
}
