use std::error::Error;

use raytracer::{
    parser,
    ray_trace,
    World,
    camera::{self, Radians},
    image::{Framebuffer, write_image},
    materials::MaterialType,
    maths::{Vec3, IVector, Y_AXIS},
    Options
};


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
    let options = Options{ samples_per_pixel, max_ray_bounces };

    let _color1 = MaterialType::Diffuse(Vec3::new(1.0, 0.0, 1.0));
    let _color2 = MaterialType::Emission(Vec3::new(0.0, 1.0, 1.0));
    let _color3 = MaterialType::Dielectric(1.5);


    let (_camera, spheres) = parser::parse_world()?;
    let world = World::new(
        spheres,
        vec![
            // Mesh::new(
            //     vec![
            //         Triangle::new(
            //             Vec3::new(-0.1, -0.1, -0.5),
            //             Vec3::new( 0.1, -0.1, -0.5),
            //             Vec3::new(-0.1,  0.1, -0.5),
            //             &color3
            //         ),
            //         Triangle::new(
            //             Vec3::new(-0.1,  0.1, -0.5),
            //             Vec3::new( 0.1, -0.1, -0.5),
            //             Vec3::new( 0.1,  0.1, -0.5),
            //             &color3
            //         )
            //     ]
            // )
        ],
    );

    // let camera = camera::Camera::new_at(Vec3::new(0.0, 0.0, 0.0), 1.77778);
    // let camera = camera::Camera::new_with_vertical_fov(Vec3::new_zero(), Radians(std::f32::consts::PI / 2.0), 1.77778);
    let camera = camera::Camera::new_look_at(
        Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Y_AXIS.into(), Radians(std::f32::consts::PI / 2.0), 1.77778
    );

    let aspect_ratio = camera.aspect_ratio();
    let image_width  = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let framebuffer = Framebuffer::new(image_width, image_height);
    let framebuffer = ray_trace(world, camera, framebuffer, &options);


    eprint!(" Done!\nWriting image...");
    write_image(&framebuffer, Some("image.ppm"))?;
    eprint!("          Done!\n");

    return Ok(());
}
