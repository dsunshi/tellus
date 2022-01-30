use crate::color_map::ColorMap;
use crate::noise_map::NoiseMap;
use create_vox::{VoxFile, Voxel};

pub struct MeshMap<'a> {
    zscale: f64,
    ground: u8,
    pub map: Vec<Vec<u8>>,
    color_map: Option<&'a ColorMap>,
    noise_map: Option<&'a NoiseMap>,
}

// Simple activation function
// Examples: https://github.com/sjeohp/activation/blob/master/src/lib.rs
fn height_curve(x: f64) -> f64 {
    let c = 4.5;
    // if x < 0.23 {
    //     return 0.23;
    // }

    (1.1 + ((x - 1.0) * c).exp()).ln()
}

impl<'a> MeshMap<'a> {
    pub fn new(width: u32, height: u32) -> Self {
        MeshMap {
            zscale: 1.0,
            ground: 0,
            map: vec![vec![0; width as usize]; height as usize],
            color_map: None,
            noise_map: None,
        }
    }

    pub fn zscale(mut self, zscale: f64) -> Self {
        self.zscale = zscale;
        self
    }

    // TODO: Why is the ground offset needed?
    pub fn ground(mut self, ground: u8) -> Self {
        self.ground = ground;
        self
    }

    pub fn color(mut self, color_map: &'a ColorMap) -> Self {
        self.color_map = Some(color_map);
        self
    }

    pub fn noise(mut self, noise_map: &'a NoiseMap) -> Self {
        self.noise_map = Some(noise_map);
        self
    }

    pub fn build(mut self) -> Result<Self, String> {
        if self.color_map.is_none() {
            return Err("ColorMap must be specified before mesh can be generated!".to_string());
        }
        if self.noise_map.is_none() {
            return Err("NoiseMap must be specified before mesh can be generated!".to_string());
        }

        let noise_map = self.noise_map.unwrap();

        for y in 0..noise_map.height {
            for x in 0..noise_map.width {
                // Get the noise height from the noise map
                let noise_height = noise_map.map[x as usize][y as usize];
                // Apply the activation function
                // TODO: How to specify this as closure?
                let noise_height = height_curve(noise_height);
                // TODO: Why is the ground offset needed?
                let z = ((noise_height * self.zscale) as u8) + self.ground;
                self.map[x as usize][y as usize] = z;
            }
        }

        Ok(self)
    }

    pub fn render(&self, vox: &mut VoxFile) -> Result<(), String> {
        // TODO: build checks this, but how to know render was called after build?
        let noise_map = self.noise_map.unwrap();
        let color_map = self.color_map.unwrap();

        for y in 0..noise_map.height {
            for x in 0..noise_map.width {
                let z = self.map[x as usize][y as usize];
                let color_index = color_map.get_terrain_color(z - self.ground);
                if color_index.is_none() {
                    return Err(format!("Unable to map height: {}", z - self.ground));
                }
                let color_index = color_index.unwrap();

                let voxel = Voxel::new(x as u8, y as u8, z as u8, color_index);
                if z > self.ground {
                    for zp in (self.ground..z).rev() {
                        // As we decend, recalculate the color index
                        let color_index = color_map.get_terrain_color(zp - self.ground);
                        if color_index.is_none() {
                            return Err(format!("Unable to map height: {}", z - self.ground));
                        }
                        let color_index = color_index.unwrap();
                        let voxel = Voxel::new(x as u8, y as u8, zp as u8, color_index);
                        if let Err(_) = vox.models[0].add_voxel(voxel) {
                            // TODO: What else could we do to improve this?
                            return Err(
                                "Unable to add voxel, does it fit inside the model?".to_string()
                            );
                        }
                    }
                }

                // TODO: What else could we do to improve this?
                if let Err(_) = vox.models[0].add_voxel(voxel) {
                    return Err("Unable to add voxel, does it fit inside the model?".to_string());
                }
            }
        }

        Ok(())
    }
}
