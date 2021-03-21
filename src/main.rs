use std::error::Error;

pub mod maths;
pub mod parser;
pub mod camera;
pub mod image;
pub mod random;
pub mod mat3;
pub mod materials;
pub mod common;
pub mod color;


use materials::MaterialType;
use image::{Framebuffer, write_image};
use camera::Radians;
use maths::{Vec3, IVector, Y_AXIS};
use common::{World, Options, Mesh, Triangle, ray_trace};
use std::io::stderr;
use crate::color::Color;


fn get_arguments() -> Result<(i32, i32), Box<dyn Error>> {
    let mut samples_per_pixel = 50;
    let mut max_ray_bounces   = 8;

    let mut arguments = std::env::args();
    let _path = arguments.next();

    for argument in arguments {
        if let Ok(next) = parser::starts_with(&argument, "samples") {
            let next = parser::starts_with(next, "=")?;
            let (_, samples) = parser::parse_int(next)?;
            samples_per_pixel = samples;
        } else if let Ok(next) = parser::starts_with(&argument, "ray_depth") {
            let next = parser::starts_with(next, "=")?;
            let (_, ray_depth) = parser::parse_int(next)?;
            max_ray_bounces = ray_depth;
        } else {
            panic!("Unknown argument '{}'", argument);
        }
    }

    Ok((samples_per_pixel, max_ray_bounces))
}


fn main() -> Result<(), Box<dyn Error>> {
    let (samples_per_pixel, max_ray_bounces) = get_arguments()?;
    eprintln!("Using:\n* Samples per pixel: {}\n* Max ray depth: {}", samples_per_pixel, max_ray_bounces);
    let mut options = Options::new(samples_per_pixel, max_ray_bounces, Some(Box::new(stderr())), true);

    let color1 = MaterialType::Diffuse(Color::new(1.0, 0.0, 1.0));
    let color2 = MaterialType::Emission(Color::new(0.0, 1.0, 1.0));
    let color3 = MaterialType::Dielectric(1.5);

    let (_camera, spheres) = parser::parse_world()?;
    let world = World::new(
        spheres,
        vec![
            Mesh::new(
                vec![
                    // Front face
                    Triangle::new(
                        Vec3::new(-0.1, -0.1, -0.5),
                        Vec3::new( 0.1, -0.1, -0.5),
                        Vec3::new(-0.1,  0.1, -0.5),
                        &color1
                    ),
                    Triangle::new(
                        Vec3::new(-0.1,  0.1, -0.5),
                        Vec3::new( 0.1, -0.1, -0.5),
                        Vec3::new( 0.1,  0.1, -0.5),
                        &color2
                    )
                ]
            )
        ],
    );

    // let camera = camera::Camera::new_at(Vec3::new(0.0, 0.0, 0.0), 1.77778);
    // let camera = camera::Camera::new_with_vertical_fov(Vec3::new_zero(), Radians(std::f32::consts::PI / 2.0), 1.77778);
    // let camera = camera::Camera::new_look_at(  // TODO(ted): BUG when z values are the same!
    //     Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Y_AXIS.into(), Radians(std::f32::consts::PI / 2.0), 1.77778
    // );
    let camera = camera::Camera::new_look_at(
        Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Y_AXIS.into(), Radians(std::f32::consts::PI / 2.0), 1.77778
    );

    let aspect_ratio = camera.aspect_ratio();
    let image_width  = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let framebuffer = Framebuffer::new(image_width, image_height);
    let framebuffer = ray_trace(&world, &camera, framebuffer, &mut options);


    eprint!(" Done!\nWriting image...");
    write_image(&framebuffer, Some("image.ppm"))?;
    eprint!("          Done!\n");

    return Ok(());
}
