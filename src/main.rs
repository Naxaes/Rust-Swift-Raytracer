#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::error::Error;
use std::io::Write;
use std::num::{Wrapping, NonZeroU32};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

mod maths;
mod parser;
mod camera;

use camera::{Camera, Radians};
use maths::{Vec3, Point, NVec3, reflect, refract, IVector};
use crate::maths::Y_AXIS;


//
/* https://en.wikipedia.org/wiki/Netpbm#PPM_example
P3
3 2
255
# The part above is the header
# "P3" means this is a RGB color image in ASCII
# "3 2" is the width and height of the image in pixels
# "255" is the maximum value for each color
# The part below is image data: RGB triplets
255   0   0  # red
  0 255   0  # green
  0   0 255  # blue
255 255   0  # yellow
255 255 255  # white
  0   0   0  # black
*/




// ----------------- RAY ----------------------
pub struct Ray {
    origin: Point,
    direction: NVec3,
}

impl Ray {
    fn new(origin: Point, direction: NVec3) -> Self { Self { origin, direction } }
    fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}



// ----------------- OTHER ----------------------
struct Random {
    state: Wrapping<u32>,
}

impl Random {
    fn new(seed: NonZeroU32) -> Random {
        Self { state: Wrapping(seed.get()) }
    }
    /// Random number between [0, 1].
    fn random_f32(&mut self) -> f32 {
        self.xor_shift_32() as f32 / u32::MAX as f32
    }
    /// Random number between [-1, 1].
    fn random_bilateral_f32(&mut self) -> f32 {
        self.random_f32() * 2.0 - 1.0
    }
    fn xor_shift_32(&mut self) -> u32
    {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x.0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_is_between_0_and_1() {
        let x = u32::MAX as f32 / u32::MAX as f32;
        assert!(0.0 <= x && x <= 1.0);

        let y = 0.0 / u32::MAX as f32;
        assert!(0.0 <= y && y <= 1.0);
    }
    #[test]
    fn test_is_between_minus_1_and_1() {
        let x = (u32::MAX as f32 / u32::MAX as f32) * 2.0 - 1.0;
        assert!(-1.0 <= x && x <= 1.0);

        let y = (0.0 / u32::MAX as f32) * 2.0 - 1.0;
        assert!(-1.0 <= y && y <= 1.0);
    }
}


fn lerp<T>(a: T, b: T, t: f32) -> T
    where T: std::ops::Mul<f32, Output=T> + std::ops::Add<T, Output=T> {
    a*(1.0-t) + b*t
}


fn random_unit_sphere(random: &mut Random) -> NVec3 {
    NVec3::new(
        random.random_bilateral_f32(),
        random.random_bilateral_f32(),
        random.random_bilateral_f32(),
    )
}


// ----------------- FRAMEBUFFER ----------------------
#[derive(Debug, Clone)]
struct Framebuffer {
    max_color_value: usize,
    width:  usize,
    height: usize,
    pixels: Vec<Vec3>,
}

impl Framebuffer {
    fn new(width: usize, height: usize) -> Self {
        let max_color_value = 255;

        let mut pixels : Vec<Vec3> = Vec::with_capacity(width * height);
        pixels.resize(width * height, Vec3 { x: 0.0, y: 0.0, z: 0.0 });
        Self { max_color_value, width, height, pixels }
    }
}

impl std::ops::Index<[usize; 2]> for Framebuffer {
    type Output = Vec3;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [row, column] = index;
        &self.pixels[row * self.width + column]
    }
}

impl std::ops::IndexMut<[usize; 2]> for Framebuffer {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [row, column] = index;
        &mut self.pixels[row * self.width + column]
    }
}

fn write_image(framebuffer: &Framebuffer) {
    print!(
        "P3\n{width} {height}\n{max_color_value}\n",
        width=framebuffer.width, height=framebuffer.height, max_color_value=framebuffer.max_color_value
    );

    for row in (0usize..framebuffer.height).rev() {
        for column in 0usize..framebuffer.width {
            let color = framebuffer[[row, column]];
            print!("{} {} {}\n", color.x as u32, color.y as u32, color.z as u32);
        }
    }
}


// ----------------- MATERIALS ----------------------
#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    Diffuse(Vec3),
    Metal(Vec3, f32),
    Dielectric(f32),
}

trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)>;
}

fn hit_front_face(direction: &Vec3, normal: &NVec3) -> bool {
    direction.dot(normal) >= 0.0
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)> {
        match self {
            MaterialType::Diffuse(color)     => diffuse_scatter(*color, ray, hit, random),
            MaterialType::Metal(color, fuzz) => metal_scatter(*color, *fuzz, ray, hit, random),
            MaterialType::Dielectric(ir)     => dielectric_scatter(*ir, ray, hit, random),
        }
    }
}

fn diffuse_scatter(color: Vec3, ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)> {
    let scatter = hit.normal + random_unit_sphere(random);

    // Catch degenerate scatter direction
    if scatter.near_zero() {
        Some((color, Ray::new(hit.position, hit.normal)))
    } else {
        Some((color, Ray::new(hit.position, scatter.normalize())))
    }
}

fn metal_scatter(color: Vec3, fuzz: f32, ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)> {
    let reflected = reflect(ray.direction.into(), hit.normal);
    let direction = reflected + fuzz*random_unit_sphere(random);

    if hit_front_face(&direction, &hit.normal) {
        Some((color, Ray::new(hit.position, direction.normalize())))
    } else {
        None
    }
}

fn dielectric_scatter(ir: f32, ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)> {
    let (normal, refraction_ratio) =
        if hit_front_face(&ray.direction.into(), &hit.normal) {
            (-hit.normal, 1.0/ir)  // Ray is inside the object.
        } else {
            (hit.normal, ir)      // Ray is outside the object.
        };

    // let cos_theta = NVec3::dot(&-ray.direction, &hit.normal);
    // let sin_theta = f32::sqrt(1.0 - cos_theta*cos_theta);
    //
    // let cannot_refract = refraction_ratio * sin_theta < 1.0;
    // let direction =
    //     if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f32(&random) {
    //         reflect(ray.direction.into(), hit.normal)
    //     } else {
    //         refract(ray.direction, hit.normal, refraction_ratio)
    //     };

    let refracted = refract(ray.direction, normal, refraction_ratio);
    let scattered = Ray::new(hit.position, refracted.normalize());

    Some((Vec3::new(1.0, 1.0, 1.0), scattered))
}


fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0-refraction_ratio) / (1.0+refraction_ratio);
    let r0 = r0*r0;
    r0 + (1.0-r0) * f32::powi(1.0 - cos_theta, 5)
}


// ----------------- HITTABLES ----------------------
struct HitRecord<'a> {
    position: Point,
    normal: NVec3,
    t: f32,
    material: &'a MaterialType,
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Point,
    radius: f32,
    material: MaterialType,
}
impl Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // let oc = ray.origin - self.center;
        // let a  = ray.direction.length_squared();
        // let b  = 2.0 * oc.dot(&ray.direction);
        // let c  = oc.length_squared() - self.radius.powi(2);
        // let discriminant = b.powi(2) - 4.0*a*c;

        // if discriminant < 0.0 {
        //     return None;
        // }
        //
        // let root1 = (-b - discriminant.sqrt()) / (2.0*a);
        // let root2 = (-b + discriminant.sqrt()) / (2.0*a);

        let oc = ray.origin - self.center;
        let a  = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let root1 = (-half_b - discriminant.sqrt()) / a;
        let root2 = (-half_b + discriminant.sqrt()) / a;

        let t = if t_min < root1 && root1 < t_max {
            root1
        } else if t_min < root2 && root2 < t_max {
            root2
        } else {
            return None
        };

        let position = ray.at(t);
        let normal   = ((position - self.center) / self.radius).normalize();

        return Some(HitRecord{ t, position, normal, material: &self.material });
    }
}

// #[derive(Debug, Copy, Clone)]
// pub struct Plane {
//     center: Point,
//     tangent1: Vec3,
//     tangent2: Vec3,
// }
// impl Plane {
//     fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//
//     }
// }

struct World {
    spheres: Vec<Sphere>,
}

impl World {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest = f32::INFINITY;
        let mut hit_record : Option<HitRecord> = None;

        for sphere in &self.spheres {
            let hit = sphere.hit(&ray, 0.001, closest);
            if let Some(h) = hit {
                closest = h.t;
                hit_record = Some(h);
            }
        }

        hit_record
    }
}



fn ray_color(ray: &Ray, world: &World, random: &mut Random, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray) {
        let material = hit.material;
        return if let Some((color, new_ray)) = material.scatter(ray, &hit, random) {
            color * ray_color(&new_ray, world, random, depth - 1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        // Sky
        let t = 0.5 * (ray.direction.normalize().y() + 1.0);
        lerp(Vec3{ x: 1.0, y: 1.0, z: 1.0 }, Vec3{ x: 0.5, y: 0.7, z: 1.0 }, t)
    }
}


fn get_arguments() -> Result<(i32, i32), Box<dyn Error>> {
    let mut samples_per_pixel = 50;
    let mut max_ray_bounces   = 8;

    for argument in std::env::args() {
        if let Ok(next) = parser::starts_with(&argument, "samples") {
            let next = parser::starts_with(next, "=")?; // ("Expected '=' after 'samples'.")?;
            let (_, samples) = parser::parse_int(next)?; // ("Couldn't parse int after 'samples='.")?;
            samples_per_pixel = samples;
        }
        if let Ok(next) = parser::starts_with(&argument, "ray_depth") {
            let next = parser::starts_with(next, "=")?; // .ok_or("Expected '=' after 'ray_depth'.")?;
            let (_, ray_depth) = parser::parse_int(next)?; // .ok_or("Couldn't parse int after 'ray_depth='.")?;
            max_ray_bounces = ray_depth;
        }
    }

    Ok((samples_per_pixel, max_ray_bounces))
}


fn main() -> Result<(), Box<dyn Error>> {

    let (samples_per_pixel, max_ray_bounces) = get_arguments()?;
    eprintln!("Using:\n* Samples per pixel: {}\n* Max ray depth: {}", samples_per_pixel, max_ray_bounces);

    let (camera, spheres) = parser::parse_world()?;
    let world = World { spheres };

    // let camera = camera::Camera::new_at(Vec3::new(0.0, 0.0, 0.0), 1.77778);
    // let camera = camera::Camera::new_with_vertical_fov(Vec3::new_zero(), Radians(std::f32::consts::PI / 2.0), 1.77778);
    let camera = camera::Camera::new_look_at(
        Vec3::new(2.0, 1.0, 3.0), Vec3::new(0.0, 0.0, -1.0), Y_AXIS.into(), Radians(std::f32::consts::PI / 2.0), 1.77778
    );

    let mut random = Random::new(NonZeroU32::new(245).unwrap());

    // Image
    let aspect_ratio = camera.aspect_ratio();
    let image_width  = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let mut framebuffer = Framebuffer::new(image_width, image_height);

    for row in 0..framebuffer.height {
        eprint!("\rScanlines remaining: {:<4}", framebuffer.height-1 - row);
        std::io::stdout().flush().unwrap();

        for column in 0..framebuffer.width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (column as f32 + random.random_f32()) / (framebuffer.width-1)  as f32;
                let v = (row    as f32 + random.random_f32()) / (framebuffer.height-1) as f32;
                let ray = camera.cast_ray(u, v);
                color = color + ray_color(&ray, &world, &mut random, max_ray_bounces);
            }

            // Gamma correction (approximate to sqrt).
            let rgb = Vec3::new(
                f32::sqrt(color.x * (1.0 / samples_per_pixel as f32)) * 255.999,
                f32::sqrt(color.y * (1.0 / samples_per_pixel as f32)) * 255.999,
                f32::sqrt(color.z * (1.0 / samples_per_pixel as f32)) * 255.999,
            );
            framebuffer[[row, column]] = rgb;
        }
    }

    eprint!(" Done!\nWriting image...");
    write_image(&framebuffer);
    eprint!("          Done!\n");

    return Ok(());
}
