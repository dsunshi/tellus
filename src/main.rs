use create_vox::{Color, VoxFile, Voxel};
use noise::{NoiseFn, OpenSimplex};
use rand::prelude::*;

struct Map {
    width: u32,
    height: u32,
    noise_map: Vec<Vec<f64>>,
    scale: f64,
    octaves: u32,
    persistance: f64,
    lacunarity: f64,
    offset: Vector2D,
}

struct Vector2D {
    x: i32,
    y: i32,
}

fn inverselerp(a: f64, b: f64, x: f64) -> f64 {
    (x - a) / (b - a)
}

impl Map {
    fn new(width: u32, height: u32) -> Self {
        Map {
            width,
            height,
            noise_map: vec![vec![0.0; width as usize]; height as usize],
            scale: 0.0001,
            octaves: 0,
            persistance: 0.0001,
            lacunarity: 0.0001,
            offset: Vector2D { x: 0, y: 0 },
        }
    }

    fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    fn octaves(mut self, octaves: u32) -> Self {
        self.octaves = octaves;
        self
    }

    fn persistance(mut self, persistance: f64) -> Self {
        self.persistance = persistance;
        self
    }

    fn lacunarity(mut self, lacunarity: f64) -> Self {
        self.lacunarity = lacunarity;
        self
    }

    fn offset(mut self, x: i32, y: i32) -> Self {
        self.offset = Vector2D { x, y };
        self
    }

    fn build(mut self) -> Result<Self, String> {
        // Generate a 2D vector the size of width and height
        let noise = OpenSimplex::new();

        // Check the inputs
        if self.scale <= 0.0 {
            return Err("Scale must be greater than zero!".to_string());
        }

        // Generate random offsets for each octave
        let mut rng = thread_rng();
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
                    // scaled x = x / scale * frequency + octave offset
                    let sx = ((x as f64) / self.scale) * frequency
                        + (octave_offsets[i as usize].x as f64);
                    let sy = ((y as f64) / self.scale) * frequency
                        + (octave_offsets[i as usize].y as f64);

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
                // TODO: How to handle z-scale?
                self.noise_map[x as usize][y as usize] = noise_height * 20.0;
                assert!(noise_height < 255.0);
            }
        }

        // Normalize
        for y in 0..self.height {
            for x in 0..self.width {
                self.noise_map[x as usize][y as usize] = inverselerp(
                    min_noise_height,
                    max_noise_height,
                    self.noise_map[x as usize][y as usize],
                );
                assert!(self.noise_map[x as usize][y as usize] < 255.0);
            }
        }

        Ok(self)
    }

    // TODO: Why is the ground offset needed?
    fn fill(&self, vox: &mut VoxFile, ground: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let z = (self.noise_map[x as usize][y as usize] as u32) + ground;
                let voxel = Voxel::new(x as u8, y as u8, z as u8, 1);
                if z > ground {
                    for zp in (ground..z).rev() {
                        let voxel = Voxel::new(x as u8, y as u8, zp as u8, 1);
                        vox.models[0].add_voxel(voxel);
                    }
                }

                vox.models[0].add_voxel(voxel);
            }
        }
    }

    // fn draw_noise_map(self) {}
}

fn main() {
    let mut vox = VoxFile::new(100, 100, 40);
    let map = Map::new(100, 100)
        .scale(27.3)
        .octaves(4)
        .persistance(0.5)
        .lacunarity(2.0)
        .offset(0, 0)
        .build()
        .expect("WTF");
    vox.set_palette_color(1, 129, 108, 91, 255);
    map.fill(&mut vox, 20);
    // for x in 0..100 {
    //     for y in 0..100 {
    //         let z = (map.noise_map[x][y] + 20.0) as u32;
    //         let voxel = Voxel::new(x as u32, y as u32, z, 1);
    //         vox.models[0].add_voxel(voxel);
    //     }
    // }

    vox.save("firma.vox");
}
