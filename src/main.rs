use create_vox::{Color, VoxFile, Voxel};
use noise::{NoiseFn, OpenSimplex};

struct Map {
    width: u8,
    height: u8,
    noise_map: Vec<Vec<f64>>,
    scale: f64,
    octaves: u8,
    persistance: f64,
    lacunarity: f64,
}

fn inverselerp(x: f64, a: f64, b: f64) -> f64 {
    (x - a) / (b - a)
}

impl Map {
    fn new(width: u8, height: u8) -> Self {
        Map {
            width,
            height,
            noise_map: vec![vec![0.0; width.into()]; height.into()],
            scale: 0.0001,
            octaves: 0,
            persistance: 0.0001,
            lacunarity: 0.0001,
        }
    }

    fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    fn octaves(mut self, octaves: u8) -> Self {
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

    fn build(mut self) -> Result<Self, String> {
        // Generate a 2D vector the size of width and height
        let noise = OpenSimplex::new();
        if self.scale <= 0.0 {
            return Err("Scale must be greater than zero!".to_string());
        }

        let mut max_noise_height = f64::MIN;
        let mut min_noise_height = f64::MAX;

        for y in 0..self.height {
            for x in 0..self.width {
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                let mut noise_height = 1.0;
                for _i in 0..self.octaves {
                    let sx = ((x as f64) / self.scale) * frequency;
                    let sy = ((y as f64) / self.scale) * frequency;
                    let sz = noise.get([sx, sy]);

                    noise_height += sz * amplitude;

                    amplitude *= self.persistance;
                    frequency *= self.lacunarity;
                }

                if noise_height > max_noise_height {
                    max_noise_height = noise_height;
                } else if noise_height < min_noise_height {
                    min_noise_height = noise_height;
                }
                self.noise_map[x as usize][y as usize] = noise_height;
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
            }
        }

        Ok(self)
    }

    fn draw_noise_map(self) {}
}

fn main() {
    let mut vox = VoxFile::new(100, 100, 40);
    let map = Map::new(100, 100)
        .scale(24.3)
        .octaves(4)
        .persistance(0.5)
        .lacunarity(2.0)
        .build()
        .expect("WTF");
    vox.set_palette_color(1, 129, 108, 91, 255);
    for x in 0..100 {
        for y in 0..100 {
            let z = (map.noise_map[x][y] + 20.0) as u8;
            let voxel = Voxel::new(x as u8, y as u8, z, 1);
            vox.models[0].add_voxel(voxel);
        }
    }

    vox.save("firma.vox");
}
