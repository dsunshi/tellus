use create_vox::{Color, VoxFile, Voxel};
use noise::{NoiseFn, OpenSimplex};
use rand::prelude::*;
use std::cmp::Ordering;

struct NoiseMap {
    width: u32,
    height: u32,
    map: Vec<Vec<f64>>,
    scale: f64,
    octaves: u32,
    persistance: f64,
    lacunarity: f64,
    offset: Vector2D,
}

struct MeshMap {
    zscale: f64,
    ground: u32,
}

impl MeshMap {
    fn new(zscale: f64, ground: u32) -> Self {
        MeshMap { zscale, ground }
    }

    // TODO: Why is the ground offset needed?
    fn render(&self, vox: &mut VoxFile, color_map: &ColorMap, noise_map: &NoiseMap) {
        for y in 0..noise_map.height {
            for x in 0..noise_map.width {
                let noise_height = noise_map.map[x as usize][y as usize];
                let color_index = color_map.map[x as usize][y as usize];
                let z = ((noise_height * self.zscale) as u32) + self.ground;
                let voxel = Voxel::new(x as u8, y as u8, z as u8, color_index);
                if z > self.ground {
                    for zp in (self.ground..z).rev() {
                        let voxel = Voxel::new(x as u8, y as u8, zp as u8, color_index);
                        vox.models[0].add_voxel(voxel);
                    }
                }

                vox.models[0].add_voxel(voxel);
            }
        }
    }
}

struct Terrain {
    name: String,
    height: f64,
    color_index: u8,
}

impl PartialOrd for Terrain {
    // Tick should be sorted by the timestamp
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.height.partial_cmp(&other.height)
    }
}

impl Ord for Terrain {
    // Tick should be sorted by the timestamp
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.partial_cmp(&other.height).unwrap()
    }
}

impl PartialEq for Terrain {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Terrain {}

struct ColorMap {
    width: u32,
    height: u32,
    colors: Vec<Terrain>,
    map: Vec<Vec<u8>>,
}

struct Vector2D {
    x: i32,
    y: i32,
}

fn inverselerp(a: f64, b: f64, x: f64) -> f64 {
    (x - a) / (b - a)
}

impl ColorMap {
    fn new(width: u32, height: u32) -> Self {
        ColorMap {
            width,
            height,
            colors: Vec::new(),
            map: vec![vec![0; width as usize]; height as usize],
        }
    }

    fn add(&mut self, terrain: Terrain) {
        self.colors.push(terrain);
        self.colors.sort();
    }

    fn apply_noise_map(&mut self, noise_map: &NoiseMap) {
        for y in 0..noise_map.height {
            for x in 0..noise_map.width {
                let noise_height = noise_map.map[x as usize][y as usize];
                for color in &self.colors {
                    if noise_height < color.height {
                        self.map[x as usize][y as usize] = color.color_index;
                        break;
                    }
                }
            }
        }
    }
}

impl NoiseMap {
    fn new(width: u32, height: u32) -> Self {
        NoiseMap {
            width,
            height,
            map: vec![vec![0.0; width as usize]; height as usize],
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

    // fn draw_noise_map(self) {}
}

fn main() {
    let width = 100;
    let height = 100;
    let mut vox = VoxFile::new(width, height, 40);
    let noise_map = NoiseMap::new(width.into(), height.into())
        .scale(27.3)
        .octaves(4)
        .persistance(0.5)
        .lacunarity(2.0)
        .offset(0, 0)
        .build()
        .expect("WTF");
    let mut color_map = ColorMap::new(width.into(), height.into());
    let mesh_map = MeshMap::new(15.0, 20);

    // colors
    // water - rgb(0, 121, 150)
    vox.set_palette_color(2, 0, 121, 150, 255);
    let water = Terrain {
        name: "water".to_string(),
        height: 0.25,
        color_index: 2,
    };
    color_map.add(water);
    // dirt - rgb(129, 108, 91)
    vox.set_palette_color(1, 129, 108, 91, 255);
    let dirt = Terrain {
        name: "dirt".to_string(),
        height: 0.4,
        color_index: 1,
    };
    color_map.add(dirt);
    // grass - rgb(102, 141, 60)
    vox.set_palette_color(3, 102, 141, 60, 255);
    let grass = Terrain {
        name: "grass".to_string(),
        height: 0.7,
        color_index: 3,
    };
    color_map.add(grass);
    // snow  - rgb(231, 227, 215)
    vox.set_palette_color(4, 231, 227, 215, 255);
    let snow = Terrain {
        name: "snow".to_string(),
        height: 1.6,
        color_index: 4,
    };
    color_map.add(snow);

    // Map the colors to the noise
    color_map.apply_noise_map(&noise_map);

    mesh_map.render(&mut vox, &color_map, &noise_map);
    // noise_map.fill(&mut vox, 20);
    // for x in 0..100 {
    //     for y in 0..100 {
    //         let z = (map.noise_map[x][y] + 20.0) as u32;
    //         let voxel = Voxel::new(x as u32, y as u32, z, 1);
    //         vox.models[0].add_voxel(voxel);
    //     }
    // }

    vox.save("firma.vox");
}
