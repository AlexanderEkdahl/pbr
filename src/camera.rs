use rand::{random, Rand, Rng, ThreadRng};
use cgmath::{Point3, Vector3, Matrix4, BaseNum, ApproxEq, BaseFloat};
use cgmath::{ElementWise, InnerSpace};
use num::traits::{zero, Zero, one, One, FloatConst, FromPrimitive};
use ray::Ray;

#[derive(Debug)]
pub struct Camera<T> {
    pub width: usize,
    pub height: usize,
    lens: T,
    sensor: T,
    f_stop: T,
    focus: T,
    pos: Matrix4<T>,
}

impl<T: BaseNum + BaseFloat + ApproxEq + Zero + One + Rand + FloatConst + FromPrimitive> Camera<T> {
    // 35mm
    // lens 0.050
    // sensor: 0.024
    // f_stop: 4.0
    // position should have a z of 3..
    pub fn new(width: usize, height: usize, lens: T, sensor: T, f_stop: T) -> Camera<T> {
        let position = Point3::new(zero::<T>(), zero::<T>(), one::<T>());
        let target = Point3::new(zero::<T>(), zero::<T>(), zero::<T>());
        let focus = target.clone();

        Camera {
            width: width,
            height: height,
            lens: lens,
            sensor: sensor,
            focus: (focus - position).magnitude(),
            f_stop: f_stop,
            pos: Matrix4::look_at(position, target, Vector3::new(zero::<T>(), one::<T>(), zero::<T>())),
        }
    }

    // pub fn ray(&self, x: f64, y: f64, rng: &mut ThreadRng) -> Ray<f64> {
    //     let Closed01(val) = random::<Closed01<f64>>();
    //     let rx = x + val;
    //     let Closed01(val) = random::<Closed01<f64>>();
    //     let ry = y + val;
    //     let px = rx / self.width as f64;
    //     let py = ry / self.height as f64;
    //     let sensor_pt = self.sensor_point(px, py);
    //     let straight = &Vector3{x: 0.0, y: 0.0, z: 0.0} - &sensor_pt.unit();
    //     let focal_pt = &straight * self.focus;
    //     let lens_pt = self.aperture_point(rng);
    //     let refracted = &focal_pt - &lens_pt.unit();
    //     let ray = Ray{origin: lens_pt, direction: refracted};
    //     self.pos.mult_ray(&ray)
    // }

    pub fn sensor_point(&self, u: T, v: T) -> Point3<T> {
        let aspect = T::from_usize(self.width).unwrap() / T::from_usize(self.height).unwrap();
        let w = self.sensor * aspect;
        let h = self.sensor;
        let z = one::<T>() / ((one::<T>() / self.lens) - (one::<T>() / self.focus));
        let x = (u - one::<T>() / (one::<T>() + one::<T>())) * w;
        let y = (v - one::<T>() / (one::<T>() + one::<T>())) * h;
        Point3::new(-x, y, z)
    }

    pub fn aperture_point(&self, rng: &mut ThreadRng) -> Point3<T> {
        let d = self.lens / self.f_stop;
        let t = (one::<T>() + one::<T>()) * T::PI() * rng.gen::<T>();
        let r = (rng.gen::<T>() * d) / (one::<T>() + one::<T>());
        let x = r * t.cos();
        let y = r * t.sin();
        Point3::new(x, y, zero::<T>())
    }
}
