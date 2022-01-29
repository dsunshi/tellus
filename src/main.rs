use create_vox::{Color, VoxFile, Voxel};
use noise::{NoiseFn, OpenSimplex};

struct Map {
    width: u8,
    height: u8,
    noise_map: Vec<Vec<f64>>,
    scale: f64,
}

impl Map {
    fn new(width: u8, height: u8) -> Self {
        Map {
            width,
            height,
            noise_map: vec![vec![0.0; width.into()]; height.into()],
            scale: 0.0001,
        }
    }

    fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    fn build(mut self) -> Result<Self, String> {
        // Generate a 2D vector the size of width and height
        let noise = OpenSimplex::new();
        if self.scale <= 0.0 {
            return Err("Scale must be greater than zero!".to_string());
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let sx = (x as f64) / self.scale;
                let sy = (y as f64) / self.scale;
                let sz = noise.get([sx, sy]);
                self.noise_map[x as usize][y as usize] = sz;
            }
        }
        Ok(self)
    }

    fn draw_noise_map(self) {}
}

fn main() {
    let mut vox = VoxFile::new(100, 100, 100);
    let map = Map::new(100, 100).scale(24.3).build().expect("WTF");
    for x in 0..100 {
        for y in 0..100 {
            let z = (map.noise_map[x][y] + 20.0) as u8;
            let voxel = Voxel::new(x as u8, y as u8, z, 10);
            vox.models[0].add_voxel(voxel);
        }
    }
    vox.set_palette_color(255, 255, 0, 0, 255);

    vox.save("firma.vox");
}
