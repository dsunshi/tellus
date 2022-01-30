use crate::Vector2D;
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::prelude::*;
use rand_pcg::Pcg64;

pub struct NoiseMap {
    pub width: u32,
    pub height: u32,
    pub map: Vec<Vec<f64>>,
    scale: f64,
    octaves: u32,
    persistance: f64,
    lacunarity: f64,
    offset: Vector2D,
    seed: Option<u32>,
}

fn inverselerp(a: f64, b: f64, x: f64) -> f64 {
    (x - a) / (b - a)
}

fn rescale(x: f64, a: f64, b: f64, min: f64, max: f64) -> f64 {
    ((b - a) * (x - min)) / (max - min) + a
}

impl NoiseMap {
    pub fn new(width: u32, height: u32) -> Self {
        NoiseMap {
            width,
            height,
            map: vec![vec![0.0; width as usize]; height as usize],
            scale: 0.0001,
            octaves: 4,
            persistance: 0.5,
            lacunarity: 2.0,
            offset: Vector2D { x: 0, y: 0 },
            seed: None,
        }
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn octaves(mut self, octaves: u32) -> Self {
        self.octaves = octaves;
        self
    }

    pub fn persistance(mut self, persistance: f64) -> Self {
        self.persistance = persistance;
        self
    }

    pub fn lacunarity(mut self, lacunarity: f64) -> Self {
        self.lacunarity = lacunarity;
        self
    }

    pub fn offset(mut self, x: i32, y: i32) -> Self {
        self.offset = Vector2D { x, y };
        self
    }

    pub fn build(mut self) -> Result<Self, String> {
        let seed = if let Some(s) = self.seed {
            s
        } else {
            let mut rng = rand::thread_rng();
            rng.gen::<u32>()
        };

        // Generate a 2D vector the size of width and height
        let noise = OpenSimplex::new().set_seed(seed);

        // Check the inputs
        if self.scale <= 0.0 {
            return Err("Scale must be greater than zero!".to_string());
        }

        // Generate random offsets for each octave
        let mut rng = Pcg64::seed_from_u64(seed as u64);
        let mut octave_offsets: Vec<Vector2D> = Vec::new();
        for _i in 0..self.octaves {
            let x_offset = rng.gen_range(-100000..100000) + self.offset.x;
            let y_offset = rng.gen_range(-100000..100000) + self.offset.y;
            octave_offsets.push(Vector2D {
                x: x_offset,
                y: y_offset,
            });
        }

        let mut max_noise_height = f64::MIN;
        let mut min_noise_height = f64::MAX;

        for y in 0..self.height {
            for x in 0..self.width {
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                let mut noise_height = 1.0;
                for i in 0..self.octaves {
                    // Scaled x, y using:
                    // scaled x = ((x + octave offset) / scale) * frequency
                    let sx = (((x as i32) + octave_offsets[i as usize].x) as f64) / self.scale
                        * frequency;
                    let sy = (((y as i32) + octave_offsets[i as usize].y) as f64) / self.scale
                        * frequency;

                    // Generate the noise based on the scaled x, y above
                    let sz = noise.get([sx, sy]);

                    // This will be the z value, but this is the non-normalized version
                    noise_height += sz * amplitude;

                    amplitude *= self.persistance;
                    frequency *= self.lacunarity;
                }

                // Keep track of the min/max so it can be normalized in the next step
                if noise_height > max_noise_height {
                    max_noise_height = noise_height;
                } else if noise_height < min_noise_height {
                    min_noise_height = noise_height;
                }
                noise_height = rescale(noise_height, 0.0, 1.0, min_noise_height, max_noise_height);
                self.map[x as usize][y as usize] = noise_height;
            }
        }

        // Normalize
        for y in 0..self.height {
            for x in 0..self.width {
                self.map[x as usize][y as usize] = inverselerp(
                    min_noise_height,
                    max_noise_height,
                    self.map[x as usize][y as usize],
                );
            }
        }

        Ok(self)
    }
}
