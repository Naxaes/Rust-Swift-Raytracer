#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::error::Error;
use std::io::Write;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

mod maths;
mod parser;
mod camera;
mod image;
mod random;
mod mat3;

use mat3::Mat3;
use random::Random;
use image::{Framebuffer, write_image};
use camera::{Camera, Radians};
use maths::{Vec3, Point, NVec3, reflect, refract, IVector, Y_AXIS};







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

trait Renderable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Point,
    radius: f32,
    material: MaterialType,
}
impl Renderable for Sphere {
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

struct Triangle<'a> {
    v0 : Vec3,
    v1 : Vec3,
    v2 : Vec3,
    normal   : NVec3,
    material : &'a MaterialType,
}
pub enum Intersection {
    Intersect,
    Beside,
    Parallel,
    Behind,
}

impl<'a> Triangle<'a> {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: &'a MaterialType) -> Self {
        let a = v1 - v0;
        let b = v2 - v0;
        let n = a.cross(&b).normalize();
        Self {
            v0, v1, v2, normal: n, material
        }
    }
    pub fn intersect(&self, ray: &Ray,  t_min: f32, t_max: f32) -> Option<HitRecord> {
        let Triangle { v0, v1, v2, .. } = *self;

        // -- Intersection with the triangle's coplanar plane.

        // NOTE: Not normalized as the length is significant, which
        //  is why we can't use the triangles normal field.
        let a = v1 - v0;
        let b = v2 - v0;
        let n = a.cross(&b);

        fn is_zero(a: f32) -> bool { -1e-8 < a && a < 1e-8 }

        let cos_angle_and_length = n.dot(&ray.direction);
        if is_zero(cos_angle_and_length) { return None; }  // Parallel

        let d = n.dot(&v0);
        let t = (n.dot(&ray.origin) + d) / cos_angle_and_length;
        if t < t_min || t > t_max { return None; }  // TODO: Might need to check intersection in this case.

        // -- Intersection with triangle.
        let p = ray.at(t);

        // Edge 0
        let e0  = v1 - v0;
        let vp0 = p  - v0;
        let n0  = e0.cross(&vp0);
        if n.dot(&n0) < 0.0 { return None; }

        // Edge 1
        let e1  = v2 - v1;
        let vp1 = p  - v1;
        let n1 = e1.cross(&vp1);
        if n.dot(&n1) < 0.0 { return None }

        // Edge 2
        let e2  = v0 - v2;
        let vp2 = p  - v2;
        let n2 = e2.cross(&vp2);
        if n.dot(&n2) < 0.0 { return None }

        return Some(HitRecord{ position: p, normal: self.normal, t: t, material: &self.material });
    }
}

struct Mesh<'a> {
    triangles: Vec<Triangle<'a>>,
}
impl<'a> Mesh<'a> {
    pub fn new(triangles: Vec<Triangle<'a>>) -> Self {
        Self { triangles }
    }
}
impl Renderable for Mesh<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record : Option<HitRecord> = None;
        let mut closest_intersection = f32::INFINITY;

        for triangle in self.triangles.iter() {
            if let Some(hit) = triangle.intersect(&ray, t_min, t_max) {
                if hit.t < closest_intersection {
                    closest_intersection = hit.t;
                    hit_record = Some(
                        HitRecord {
                            position:  hit.position, normal: triangle.normal, t: hit.t, material: &triangle.material
                        }
                    );
                }
            }

            //
            // let v0 = triangle.v0;
            // let v1 = triangle.v1;
            // let v2 = triangle.v2;
            //
            // let e1 = v1 - v0;
            // let e2 = v2 - v0;
            //
            // let b = ray.origin - v0;
            // let a = Mat3::new((-ray.direction).into(), e1, e2);
            //
            // if let Some(inverse) = a.inverse() {
            //     let result = inverse.mul_vec3(&b);
            //     let t = result.x;
            //     let u = result.y;
            //     let v = result.z;
            //
            //     // CLARIFY! u and v should be able to be 0, right?
            //     if 0.0 <= u && 0.0 <= v && (u + v) <= 1.0 && 0.0 <= t && t < closest_intersection {
            //         closest_intersection = t;
            //         let position = ray.at(t);
            //         hit_record = Some(
            //             HitRecord{ position, normal: triangle.normal, t, material: &triangle.material }
            //         );
            //     }
            // }
        }

        return hit_record;
    }
}


struct World<'a> {
    spheres: Vec<Sphere>,
    meshes:  Vec<Mesh<'a>>,
}

impl World<'_> {
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

        for mesh in &self.meshes {
            let hit = mesh.hit(&ray, 0.001, closest);
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
        return Vec3::new_zero();
    }

    if let Some(hit) = world.hit(ray) {
        let material = hit.material;
        return if let Some((color, new_ray)) = material.scatter(ray, &hit, random) {
            color * ray_color(&new_ray, world, random, depth - 1)
        } else {
            Vec3::new_zero()
        }
    } else {
        // Sky
        let t = 0.5 * (ray.direction.normalize().y() + 1.0);
        lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
    }
}


struct Options {
    samples_per_pixel: i32,
    max_ray_bounces:   i32,
}

fn ray_trace(world: World, camera: Camera, mut framebuffer: Framebuffer, options: &Options) -> Framebuffer {
    let mut random = Random::new();

    // Image
    for row in 0..framebuffer.height {
        eprint!("\rScanlines remaining: {:<4}", framebuffer.height-1 - row);
        std::io::stdout().flush().unwrap();

        for column in 0..framebuffer.width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..options.samples_per_pixel {
                let u = (column as f32 + random.random_f32()) / (framebuffer.width-1)  as f32;
                let v = (row    as f32 + random.random_f32()) / (framebuffer.height-1) as f32;
                let ray = camera.cast_ray(u, v);
                color = color + ray_color(&ray, &world, &mut random, options.max_ray_bounces);
            }

            // Gamma correction (approximate to sqrt).
            let rgb = Vec3::new(
                f32::sqrt(color.x * (1.0 / options.samples_per_pixel as f32)) * 255.999,
                f32::sqrt(color.y * (1.0 / options.samples_per_pixel as f32)) * 255.999,
                f32::sqrt(color.z * (1.0 / options.samples_per_pixel as f32)) * 255.999,
            );
            framebuffer[[row, column]] = rgb;
        }
    }

    framebuffer
}


fn get_arguments() -> Result<(i32, i32), Box<dyn Error>> {
    let mut samples_per_pixel = 50;
    let mut max_ray_bounces   = 8;

    for argument in std::env::args() {
        if let Ok(next) = parser::starts_with(&argument, "samples") {
            let next = parser::starts_with(next, "=")?;
            let (_, samples) = parser::parse_int(next)?;
            samples_per_pixel = samples;
        }
        if let Ok(next) = parser::starts_with(&argument, "ray_depth") {
            let next = parser::starts_with(next, "=")?;
            let (_, ray_depth) = parser::parse_int(next)?;
            max_ray_bounces = ray_depth;
        }
    }

    Ok((samples_per_pixel, max_ray_bounces))
}


fn main() -> Result<(), Box<dyn Error>> {
    let (samples_per_pixel, max_ray_bounces) = get_arguments()?;
    eprintln!("Using:\n* Samples per pixel: {}\n* Max ray depth: {}", samples_per_pixel, max_ray_bounces);
    let options = Options{ samples_per_pixel, max_ray_bounces };

    let color1 = MaterialType::Diffuse(Vec3::new(1.0, 0.0, 1.0));
    let color2 = MaterialType::Diffuse(Vec3::new(0.0, 1.0, 1.0));

    let (camera, spheres) = parser::parse_world()?;
    let world = World {
        spheres,
        meshes: vec![
            Mesh::new(
                vec![
                    Triangle::new(
                        Vec3::new(-1.0, 0.0, -2.0),
                        Vec3::new( 0.0, 0.0, -2.0),
                        Vec3::new(-1.0, 1.0, -2.0),
                        &color1
                    ),
                    Triangle::new(
                        Vec3::new(-1.0, 1.0, -2.0),
                        Vec3::new( 0.0, 0.0, -2.0),
                        Vec3::new( 0.0, 1.0, -2.0),
                        &color2
                    )
                ]
            )
        ],
    };

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





#[cfg(test)]
mod tests {

}