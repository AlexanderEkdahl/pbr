use sampler::Sampler;
use image::{ImageBuffer, ImageRgb8, PNG, Rgb};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct Renderer<'a> {
    pub sampler: &'a Sampler<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(sampler: &'a Sampler<'a>) -> Renderer<'a> {
        Renderer {
            sampler: sampler,
        }
    }

    pub fn png(&self) {
        let img = ImageBuffer::from_fn(self.sampler.cam.width as u32, self.sampler.cam.height as u32, |x, y| {
            let (x, y) = (x as usize, y as usize);
            let count = self.sampler.samples[x][y].count as f64;

            Rgb([
                color(self.sampler.samples[x][y].red / count),
                color(self.sampler.samples[x][y].green / count),
                color(self.sampler.samples[x][y].blue / count)
            ])
        });

        let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
        ImageRgb8(img).save(fout, PNG).unwrap();
    }
}

fn color(n: f64) -> u8 {
    gamma(n.min(255.0), 1.0) as u8
}

fn gamma(n: f64, g: f64) -> f64 {
    (n / 255.0).powf(1.0 / g) * 255.0
}
