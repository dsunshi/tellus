use crate::color_map::ColorMap;
use crate::noise_map::NoiseMap;
use create_vox::{VoxFile, Voxel};

pub struct MeshMap {
    zscale: f64,
    ground: u32,
}

// Simple activation function
// Examples: https://github.com/sjeohp/activation/blob/master/src/lib.rs
fn height_curve(x: f64) -> f64 {
    let c = 3.5;
    if x < 0.23 {
        return 0.12;
    }

    (1.0 + ((x - 1.0) * c).exp()).ln()
}

impl MeshMap {
    pub fn new(zscale: f64, ground: u32) -> Self {
        MeshMap { zscale, ground }
    }

    // TODO: Why is the ground offset needed?
    pub fn render(&self, vox: &mut VoxFile, color_map: &ColorMap, noise_map: &NoiseMap) {
        for y in 0..noise_map.height {
            for x in 0..noise_map.width {
                // Get the noise height from the noise map
                let noise_height = noise_map.map[x as usize][y as usize];
                // Apply the activation function
                // TODO: How to specify this as closure?
                let noise_height = height_curve(noise_height);
                let color_index = color_map.map[x as usize][y as usize];
                let z = ((noise_height * self.zscale) as u32) + self.ground;
                let voxel = Voxel::new(x as u8, y as u8, z as u8, color_index);
                if z > self.ground {
                    for zp in (self.ground..z).rev() {
                        let voxel = Voxel::new(x as u8, y as u8, zp as u8, color_index);
                        if let Err(_) = vox.models[0].add_voxel(voxel) {
                            // TODO: What else could we do to improve this?
                            println!("Unable to add voxel, does it fit inside the model?");
                        }
                    }
                }

                // TODO: What else could we do to improve this?
                if let Err(_) = vox.models[0].add_voxel(voxel) {
                    println!("Unable to add voxel, does it fit inside the model?");
                }
            }
        }
    }
}
