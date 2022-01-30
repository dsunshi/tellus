use create_vox::VoxFile;

use tellus::color_map::{ColorMap, Terrain};
use tellus::mesh_map::MeshMap;
use tellus::noise_map::NoiseMap;

fn main() {
    // Dimensions for the Voxel file
    let width = 100;
    let height = 100;
    let mut vox = VoxFile::new(width, height, 40);

    // Define the noise map
    let noise_map = NoiseMap::new(width.into(), height.into())
        .scale(20.3)
        .octaves(4)
        .persistance(0.5)
        .lacunarity(2.0)
        .offset(0, 0)
        .seed(0x6576616E)
        .build()
        .expect("All inputs are in a valid range!");

    // Define the color map
    // TODO: There needs to be a better way to specify colors
    let mut color_map = ColorMap::new(width.into(), height.into());
    // water - rgb(91, 118, 147)
    vox.set_palette_color(1, 91, 118, 147, 255);
    let water = Terrain::new("water", 1).from_levels(0, 2);
    color_map.add(water);
    // dirt - rgb(199, 191, 168)
    vox.set_palette_color(2, 199, 191, 168, 255);
    let dirt = Terrain::new("dirt", 2).from_levels(3, 3);
    color_map.add(dirt);
    // grass - rgb(83, 96, 66)
    vox.set_palette_color(3, 83, 96, 66, 255);
    let grass = Terrain::new("grass", 3).from_levels(4, 8);
    color_map.add(grass);
    // snow  - rgb(217, 213, 221)
    vox.set_palette_color(4, 217, 213, 221, 255);
    let snow = Terrain::new("snow", 4).from_levels(9, 15);
    color_map.add(snow);

    // Generate the "mesh"
    let mesh_map = MeshMap::new(width.into(), height.into())
        .zscale(20.0)
        .ground(20)
        .color(&color_map)
        .noise(&noise_map)
        .build()
        .expect("All inputs are in a valid range!");

    // Render the "mesh" to the VoxFile
    if let Err(e) = mesh_map.render(&mut vox) {
        panic!("Render error: {:?}", e);
    }

    // Save the output to file
    vox.save("tellus.vox");
}
