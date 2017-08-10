// http://www.fourmilab.ch/documents/specrend/specrend.c

#![feature(inclusive_range_syntax)]

#[derive(Debug)]
struct ColorSystem {
    x_red: f64,
    y_red: f64,
    x_green: f64,
    y_green: f64,
    x_blue: f64,
    y_blue: f64,
    x_white: f64,
    y_white: f64,
}

#[derive(Debug)]
struct RGB<'a> {
    cs: &'a ColorSystem,
    red: f64,
    green: f64,
    blue: f64,
}

impl<'a> RGB<'a> {
    fn to_hex(&self) -> String {
        let hex_red = (self.red * 255.0).round() as u8;
        let hex_green = (self.green * 255.0).round() as u8;
        let hex_blue = (self.blue * 255.0).round() as u8;
        format!("#{:X}{:X}{:X}", hex_red, hex_green, hex_blue)
    }

    fn normalize(&self) -> RGB {
        let m = self.red.max(self.green.max(self.blue));

        RGB {
            cs: self.cs,
            red: self.red / m,
            green: self.green / m,
            blue: self.blue / m,
        }
    }
}

fn xyz_to_rgb(cs: &ColorSystem, x: f64, y: f64, z: f64) -> RGB {
    let xr = cs.x_red;
    let yr = cs.y_red;
    let zr = 1.0 - (xr + yr);
    let xg = cs.x_green;
    let yg = cs.y_green;
    let zg = 1.0 - (xg + yg);
    let xb = cs.x_blue;
    let yb = cs.y_blue;
    let zb = 1.0 - (xb + yb);

    let xw = cs.x_white;
    let yw = cs.y_white;
    let zw = 1.0 - (xw + yw);

    let rx = (yg * zb) - (yb * zg);
    let ry = (xb * zg) - (xg * zb);
    let rz = (xg * yb) - (xb * yg);

    let gx = (yb * zr) - (yr * zb);
    let gy = (xr * zb) - (xb * zr);
    let gz = (xb * yr) - (xr * yb);

    let bx = (yr * zg) - (yg * zr);
    let by = (xg * zr) - (xr * zg);
    let bz = (xr * yg) - (xg * yr);

    let rw = ((rx * xw) + (ry * yw) + (rz * zw)) / yw;
    let gw = ((gx * xw) + (gy * yw) + (gz * zw)) / yw;
    let bw = ((bx * xw) + (by * yw) + (bz * zw)) / yw;

    let rx = rx / rw;
    let ry = ry / rw;
    let rz = rz / rw;

    let gx = gx / gw;
    let gy = gy / gw;
    let gz = gz / gw;

    let bx = bx / bw;
    let by = by / bw;
    let bz = bz / bw;

    RGB {
        cs: cs,
        red: (rx * x) + (ry * y) + (rz * z),
        green: (gx * x) + (gy * y) + (gz * z),
        blue: (bx * x) + (by * y) + (bz * z),
    }
}

const CIE_COLOR_MATCH: [[f64; 3]; 81] = [
        [0.0014,0.0000,0.0065], [0.0022,0.0001,0.0105], [0.0042,0.0001,0.0201],
        [0.0076,0.0002,0.0362], [0.0143,0.0004,0.0679], [0.0232,0.0006,0.1102],
        [0.0435,0.0012,0.2074], [0.0776,0.0022,0.3713], [0.1344,0.0040,0.6456],
        [0.2148,0.0073,1.0391], [0.2839,0.0116,1.3856], [0.3285,0.0168,1.6230],
        [0.3483,0.0230,1.7471], [0.3481,0.0298,1.7826], [0.3362,0.0380,1.7721],
        [0.3187,0.0480,1.7441], [0.2908,0.0600,1.6692], [0.2511,0.0739,1.5281],
        [0.1954,0.0910,1.2876], [0.1421,0.1126,1.0419], [0.0956,0.1390,0.8130],
        [0.0580,0.1693,0.6162], [0.0320,0.2080,0.4652], [0.0147,0.2586,0.3533],
        [0.0049,0.3230,0.2720], [0.0024,0.4073,0.2123], [0.0093,0.5030,0.1582],
        [0.0291,0.6082,0.1117], [0.0633,0.7100,0.0782], [0.1096,0.7932,0.0573],
        [0.1655,0.8620,0.0422], [0.2257,0.9149,0.0298], [0.2904,0.9540,0.0203],
        [0.3597,0.9803,0.0134], [0.4334,0.9950,0.0087], [0.5121,1.0000,0.0057],
        [0.5945,0.9950,0.0039], [0.6784,0.9786,0.0027], [0.7621,0.9520,0.0021],
        [0.8425,0.9154,0.0018], [0.9163,0.8700,0.0017], [0.9786,0.8163,0.0014],
        [1.0263,0.7570,0.0011], [1.0567,0.6949,0.0010], [1.0622,0.6310,0.0008],
        [1.0456,0.5668,0.0006], [1.0026,0.5030,0.0003], [0.9384,0.4412,0.0002],
        [0.8544,0.3810,0.0002], [0.7514,0.3210,0.0001], [0.6424,0.2650,0.0000],
        [0.5419,0.2170,0.0000], [0.4479,0.1750,0.0000], [0.3608,0.1382,0.0000],
        [0.2835,0.1070,0.0000], [0.2187,0.0816,0.0000], [0.1649,0.0610,0.0000],
        [0.1212,0.0446,0.0000], [0.0874,0.0320,0.0000], [0.0636,0.0232,0.0000],
        [0.0468,0.0170,0.0000], [0.0329,0.0119,0.0000], [0.0227,0.0082,0.0000],
        [0.0158,0.0057,0.0000], [0.0114,0.0041,0.0000], [0.0081,0.0029,0.0000],
        [0.0058,0.0021,0.0000], [0.0041,0.0015,0.0000], [0.0029,0.0010,0.0000],
        [0.0020,0.0007,0.0000], [0.0014,0.0005,0.0000], [0.0010,0.0004,0.0000],
        [0.0007,0.0002,0.0000], [0.0005,0.0002,0.0000], [0.0003,0.0001,0.0000],
        [0.0002,0.0001,0.0000], [0.0002,0.0001,0.0000], [0.0001,0.0000,0.0000],
        [0.0001,0.0000,0.0000], [0.0001,0.0000,0.0000], [0.0000,0.0000,0.0000]
];

// Calculate, by Planck's radiation law, the emittance of a black body
// of temperature bbTemp at the given wavelength (in metres)
fn black_body_spectrum(temperature: f64, wavelength: f64) -> f64 {
    let wlm: f64 = wavelength * 1e-9;   /* Wavelength in meters */

    (3.74183e-16 * wlm.powi(-5)) / ((1.4388e-2 / (wlm * temperature)).exp() - 1.0)
}

fn spectrum_to_xyz() -> (f64, f64, f64) {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    for (i, item) in CIE_COLOR_MATCH.iter().enumerate() {
        let lambda = (i * 5 + 380) as f64;
        let intensity = black_body_spectrum(5500.0, lambda);

        x += intensity * item[0];
        y += intensity * item[1];
        z += intensity * item[2];
    }

    let sum = x + y + z;

    (x / sum, y / sum, z / sum)
}

fn main() {
    let p3 = ColorSystem {
        x_red: 0.680,
        y_red: 0.320,
        x_green: 0.265,
        y_green: 0.690,
        x_blue: 0.150,
        y_blue: 0.060,
        x_white: 0.3127,
        y_white: 0.3290,
    };
    println!("{:?}", p3);
    let xyz = spectrum_to_xyz();
    println!("{:?}", xyz);
    let rgb = xyz_to_rgb(&p3, xyz.0, xyz.1, xyz.2);
    println!("{:?}", rgb.normalize().to_hex());
}
