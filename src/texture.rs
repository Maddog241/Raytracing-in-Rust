use crate::color::Color;
use crate::utilities;
use crate::vec3::Point3;
use image::open;
//trait Textrue
const BYTES_PER_PIXEL: usize = 3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

//struct SolidColor
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        SolidColor { color_value: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}

//struct CheckerTexture
pub struct CheckerTexture {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        CheckerTexture { even, odd }
    }

}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

impl Texture for &CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

///struct Perlin
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = vec![0.0; Self::point_count()];
        for i in 0..Self::point_count() {
            ranfloat[i] = utilities::random_double();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: Point3) -> f64 {
        let (u, v, w) = (p.x()-p.x().floor(), p.y()-p.y().floor(), p.z()-p.z().floor());
        let (i, j, k) = (p.x().floor(), p.y().floor(), p.z().floor());

        let (u, v, w) = (u*u*(3.0-2.0*u), v*v*(3.0-2.0*v), w*w*(3.0-2.0*w));

        let mut c = vec![vec![vec![0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[(
                        self.perm_x[((i as i32+di as i32) & 255) as usize] ^
                        self.perm_y[((j as i32+dj as i32) & 255) as usize] ^ 
                        self.perm_z[((k as i32+dk as i32) & 255) as usize]
                    ) as usize
                    ];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    fn point_count() -> usize {
        256
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0; Self::point_count()];

        for i in 0..Self::point_count() {
            p[i] = i as i32
        }

        Self::permute(&mut p, Self::point_count());

        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) { //randomize the preceding n element in p
        for i in (0..n).rev() {
            let target = utilities::random_int_with_bounds(0, i as i32 + 1);
            let tmp = p[i];
            p[i] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
    
    fn trilinear_interp(c: Vec<Vec<Vec<f64>>>, u: f64, v: f64,  w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64*u + (1.0-i as f64)*(1.0-u)) *
                            (j as f64*v + (1.0-j as f64)*(1.0-v)) * 
                            (k as f64*w + (1.0-k as f64)*(1.0-w as f64)) * c[i][j][k];
                }
            }
        }

        accum
    }
}

//struct NoiseTexture
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p * self.scale)
    }
}

//Struct ImageTexture
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img_buf = open(filename).unwrap().into_rgb8();
        let (width, height) = (img_buf.width() as usize, img_buf.height() as usize);
        ImageTexture {
            data: img_buf.into_raw(),
            width,
            height,
            bytes_per_scanline: width * BYTES_PER_PIXEL,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let u = utilities::clamp(u, 0.0, 1.0);
        let v = 1.0 - utilities::clamp(v, 0.0, 1.0);
    
        let mut i = (u*(self.width as f64)) as usize;
        let mut j = (v*(self.height as f64)) as usize;

        if i>=self.width { i = self.width-1; }
        if j>=self.height { j = self.height-1; }

        let color_scale = 1.0 / 255.0;
        let mut pixels = Vec::new();

        for k in 0..3 {
            pixels.push(self.data[ j*self.bytes_per_scanline + i*BYTES_PER_PIXEL + k]);
        }

        Color::new(color_scale*pixels[0] as f64, color_scale*pixels[1] as f64, color_scale*pixels[2] as f64)
    }
}