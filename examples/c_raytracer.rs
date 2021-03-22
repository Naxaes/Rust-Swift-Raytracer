// An example how to interact with the C library of rust raytracer.
// Could've just also imported the Rust functions for safety, but
// this just shows how a potential C compatible program might do it.

extern crate raytracer;

use raytracer::{
    render, load_world, CFramebuffer,
    color::ColorU8,
    image::write_image
};

use std::ptr::NonNull;

const WORLD_SOURCE: &str = "\
camera origin 0.0 0.0 0.0 aspect 1.77778;

material RED_DIFFUSE     : Diffuse color 1.0 0.0 0.0;
material GREEN_DIFFUSE   : Diffuse color 0.0 1.0 0.0;
material BLUE_DIFFUSE    : Diffuse color 0.0 0.0 1.0;
material GROUND_MATERIAL : Diffuse color 0.8 0.8 0.0;
material BALL_MATERIAL   : Diffuse color 0.7 0.3 0.3;

material METAL_MATERIAL_1 : Metal color 0.8 0.8 0.8 fuzz 0.3;
material METAL_MATERIAL_2 : Metal color 0.8 0.6 0.2 fuzz 1.0;

material MIRROR : Metal color 0.9 0.9 0.9 fuzz 0.0;
material GLASS  : Dielectric ir 1.5;

sphere center  0.0 -100.5 -1.0  radius 100.0 material GROUND_MATERIAL;

sphere center  0.0  0.0  -1.0  radius 0.5   material BALL_MATERIAL;
sphere center -1.0  0.0  -1.0  radius 0.5   material METAL_MATERIAL_1;
sphere center  1.0  0.0  -1.0  radius 0.5   material GLASS;

sphere center  0.0  1.0  -2.0  radius 0.5   material MIRROR;

sphere center -3.0  2.0  -3.0  radius 0.5   material RED_DIFFUSE;
sphere center  0.0  2.0  -3.0  radius 0.5   material GREEN_DIFFUSE;
sphere center  3.0  2.0  -3.0  radius 0.5   material BLUE_DIFFUSE;

triangle v0 -0.1 -0.1 -0.5  v1 0.1 -0.1 -0.5  v2 -0.1 0.1 -0.5  material RED_DIFFUSE;
triangle v0 -0.1  0.1 -0.5  v1 0.1 -0.1 -0.5  v2  0.1 0.1 -0.5  material GREEN_DIFFUSE;
\x00
";


fn main() {

    let width  = 200;
    let height = 200;

    let mut buffer = Vec::<ColorU8>::with_capacity(width*height);
    let pixels = NonNull::new(buffer.as_mut_ptr()).unwrap();

    let source = &*load_world(WORLD_SOURCE.as_ptr() as *const i8);

    let cframebuffer = CFramebuffer{ width, height, pixels };
    let framebuffer = render(cframebuffer, source).into();

    write_image(&framebuffer, Some("examples/image.ppm")).unwrap();
}
