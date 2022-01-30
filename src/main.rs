use create_vox::VoxFile;

use tellus::color_map::{ColorMap, Terrain};
use tellus::mesh_map::MeshMap;
use tellus::noise_map::NoiseMap;

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
        .expect("All inputs are in a valid range!");
    let mut color_map = ColorMap::new(width.into(), height.into());
    let mesh_map = MeshMap::new(15.0, 20);

    // colors
    // water - rgb(0, 121, 150)
    vox.set_palette_color(2, 0, 121, 150, 255);
    let water = Terrain::new("water", 0.25, 2);
    color_map.add(water);
    // dirt - rgb(129, 108, 91)
    vox.set_palette_color(1, 129, 108, 91, 255);
    let dirt = Terrain::new("dirt", 0.4, 1);
    color_map.add(dirt);
    // grass - rgb(102, 141, 60)
    vox.set_palette_color(3, 102, 141, 60, 255);
    let grass = Terrain::new("grass", 0.7, 3);
    color_map.add(grass);
    // snow  - rgb(231, 227, 215)
    vox.set_palette_color(4, 231, 227, 215, 255);
    let snow = Terrain::new("snow", 1.6, 4);
    color_map.add(snow);

    // Map the colors to the noise
    color_map.apply_noise_map(&noise_map);

    mesh_map.render(&mut vox, &color_map, &noise_map);

    vox.save("tellus.vox");
}
