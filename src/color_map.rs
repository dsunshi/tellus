use crate::noise_map::NoiseMap;
use std::cmp::Ordering;

pub struct Terrain {
    name: String,
    height: f64,
    color_index: u8,
}

pub struct ColorMap {
    pub width: u32,
    pub height: u32,
    colors: Vec<Terrain>,
    pub map: Vec<Vec<u8>>,
}

impl Terrain {
    pub fn new(name: &str, height: f64, color_index: u8) -> Self {
        Terrain {
            name: name.to_string(),
            height,
            color_index,
        }
    }
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

impl ColorMap {
    pub fn new(width: u32, height: u32) -> Self {
        ColorMap {
            width,
            height,
            colors: Vec::new(),
            map: vec![vec![0; width as usize]; height as usize],
        }
    }

    pub fn add(&mut self, terrain: Terrain) {
        self.colors.push(terrain);
        self.colors.sort();
    }

    pub fn apply_noise_map(&mut self, noise_map: &NoiseMap) {
        for y in 0..noise_map.height {
            for x in 0..noise_map.width {
                let noise_height = noise_map.map[x as usize][y as usize];
                for color in &self.colors {
                    if noise_height < color.height {
                        self.map[x as usize][y as usize] = color.color_index;
                        break;
                    }
                }
                // TODO: This should maybe return a result ...
                if self.map[x as usize][y as usize] == 0 {
                    println!(
                        "Failed to assign a color at height: {}",
                        noise_map.map[x as usize][y as usize]
                    );
                }

                assert!(self.map[x as usize][y as usize] > 0);
            }
        }
    }
}
