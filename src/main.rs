use create_vox::{Color, VoxFile, Voxel};

fn main() {
    let mut cube_vox = VoxFile::new(100, 100, 100);
    cube_vox.set_palette_color(255, 255, 0, 0, 255);
    cube_vox.models[0]
        .add_cube(25, 25, 25, 75, 75, 75, 255)
        .unwrap();
    cube_vox.save("red_cube.vox");

    println!("Hello, world!");
}
