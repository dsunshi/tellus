pub struct Terrain {
    #[allow(dead_code)]
    name: String, // TODO: is there a use case?
    color_index: u8,
    start_level: u8,
    end_level: u8,
}

pub struct ColorMap {
    pub width: u32,
    pub height: u32,
    colors: Vec<Terrain>,
    // pub map: Vec<Vec<u8>>,
}

impl Terrain {
    pub fn new(name: &str, color_index: u8) -> Self {
        Terrain {
            name: name.to_string(),
            start_level: 0,
            end_level: 0,
            color_index,
        }
    }

    pub fn from_levels(mut self, start: u8, end: u8) -> Self {
        self.start_level = start;
        self.end_level = end;
        self
    }

    pub fn is_terrain(&self, height: u8) -> bool {
        if height >= self.start_level && height <= self.end_level {
            return true;
        }
        false
    }
}

impl ColorMap {
    pub fn new(width: u32, height: u32) -> Self {
        ColorMap {
            width,
            height,
            colors: Vec::new(),
            // map: vec![vec![0; width as usize]; height as usize],
        }
    }

    pub fn add(&mut self, terrain: Terrain) {
        self.colors.push(terrain);
    }

    pub fn get_terrain_color(&self, height: u8) -> Option<u8> {
        for color in self.colors.iter() {
            if color.is_terrain(height) {
                return Some(color.color_index);
            }
        }
        None
    }
}
