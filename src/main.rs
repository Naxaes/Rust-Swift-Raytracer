#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io::Write;
use std::num::{Wrapping, NonZeroU32};

mod maths;
use maths::{Vector, Vec3, Point, NVec3};


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
struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
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
    fn random_f32(&mut self) -> f32 {
        self.xor_shift_32() as f32 / u32::MAX as f32
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




/// Given a quadratic equation ax^2 + bx + x = 0,
/// the solution is x = (-b ± sqrt(b*b - 4*a*c)) / (2*a)
///
/// The value inside sqrt (b*b - 4*a*c) is the discriminant.
///
/// A root doesn't exist if the discriminant is negative.
/// Exactly 1 root exist if the discriminant is zero.
/// Else, many roots exist if the discriminant is positive.
///
/// So,
///     if discriminant > 0:
///         1)  (-b - sqrt(discriminant) / (2*a)   // Root 1
///         2)  (-b + sqrt(discriminant) / (2*a)   // Root 2
///
///     if discriminant == 0:
///         1) -b/(2*a)            // Repeated roots (don't have to include the
///                                // discriminant as it's 0.
///     if discriminant < 0:
///         No real roots exist.
///
fn solve_quadratic(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>) {
    let discriminant = b*b - 4.0*a*c;

    if discriminant > 0.0 {
        let root1 = (-b - discriminant.sqrt()) / (2.0*a);
        let root2 = (-b + discriminant.sqrt()) / (2.0*a);
        (Some(root1), Some(root2))
    }
    else if discriminant == 0.0
    {
        let root1 = -b/(2.0*a);
        (Some(root1), None)
    }
    else
    {
        // No real roots.
        (None, None)
    }
}

fn hit_sphere(sphere: &Sphere, ray: &Ray) -> f32 {
    let oc = ray.origin - sphere.center;
    let a = ray.direction.length_squared();
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.length_squared() - sphere.radius.powi(2);
    let discriminant = b.powi(2) - 4.0*a*c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}


fn lerp<T>(a: T, b: T, t: f32) -> T
    where T: std::ops::Mul<f32, Output=T> + std::ops::Add<T, Output=T> {
    a*(1.0-t) + b*t
}

fn ray_color(ray: &Ray, world: &World) -> Vec3 {
    let hit = world.hit(ray);

    if let Some(h) = hit {
        let t = h.t;
        let n = h.normal;
        return 0.5 * Vec3{ x: n.x()+1.0, y: n.y()+1.0, z: n.z()+1.0 };
    }

    let t = 0.5 * (ray.direction.normalize().y() + 1.0);
    lerp(Vec3{ x: 1.0, y: 1.0, z: 1.0 }, Vec3{ x: 0.5, y: 0.7, z: 1.0 }, t)
}

#[derive(Debug, Clone)]
struct Framebuffer {
    width:  usize,
    height: usize,
    pixels: Vec<Vec3>,
}

impl Framebuffer {
    fn new(width: usize, height: usize) -> Self {
        let mut pixels : Vec<Vec3> = Vec::with_capacity(width * height);
        pixels.resize(width * height, Vec3 { x: 0.0, y: 0.0, z: 0.0 });
        Self { width, height, pixels }
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
        width=framebuffer.width, height=framebuffer.height, max_color_value=255
    );

    for row in (0usize..framebuffer.height).rev() {
        for column in 0usize..framebuffer.width {
            let color = framebuffer[[row, column]];
            print!("{} {} {}\n", color.x as u32, color.y as u32, color.z as u32);
        }
    }
}

struct HitRecord {
    position: Point,
    normal: NVec3,
    t: f32,
}




struct Sphere {
    center: Point,
    radius: f32,
}
impl Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0*a*c;

        if discriminant < 0.0 {
            return None;
        }

        let root1 = (-b - discriminant.sqrt()) / (2.0*a);
        let root2 = (-b + discriminant.sqrt()) / (2.0*a);

        let t = if t_min < root1 && root1 < t_max {
            root1
        } else if t_min < root2 && root2 < t_max {
            root2
        } else {
            return None
        };

        let position = ray.at(t);
        let normal   = ((position - self.center) / self.radius).normalize();

        return Some(HitRecord{ t, position, normal });
    }
}

struct World {
    spheres: Vec<Sphere>,
}

impl World {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest = f32::INFINITY;
        let mut hit_record : Option<HitRecord> = None;

        for sphere in &self.spheres {
            let hit = sphere.hit(&ray, 0.0, closest);
            if let Some(h) = hit {
                closest = h.t;
                hit_record = Some(h);
            }
        }

        hit_record
    }
}


fn main() {
    let mut random = Random::new(NonZeroU32::new(245).unwrap());

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width  = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let max_color_value   = 255;
    let samples_per_pixel = 1;

    // Camera
    let viewport_height = 2.0;
    let viewport_width  = aspect_ratio * viewport_height;
    let focal_length    = 1.0;

    // Viewport
    let origin     = Point{ x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vec3{ x: viewport_width, y: 0.0, z: 0.0 };
    let vertical   = Vec3{ x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{ x: 0.0, y: 0.0, z: focal_length };

    let mut framebuffer = Framebuffer::new(image_width, image_height);

    let world = World {
        spheres: vec![
            Sphere{ center: Point{ x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 },
            Sphere{ center: Point{ x: 0.0, y: -100.5, z: -1.0}, radius: 100.0},
        ]
    };

    for row in 0..framebuffer.height {
        eprint!("\rScanlines remaining: {:<4}", framebuffer.height-1 - row);
        std::io::stdout().flush().unwrap();

        for column in 0..framebuffer.width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (column as f32 + random.random_f32()) / (framebuffer.width-1)  as f32;
                let v = (row    as f32 + random.random_f32()) / (framebuffer.height-1) as f32;
                let ray = Ray { origin, direction: lower_left_corner + u*horizontal + v*vertical - origin };
                color = color + ray_color(&ray, &world);
            }

            let rgb = color * 255.999 * (1.0 / samples_per_pixel as f32);

            framebuffer[[row, column]] = rgb;
        }
    }

    eprint!(" Done!\nWriting image...");
    write_image(&framebuffer);
    eprint!("          Done!\n");
}
