use crate::maths::{Vector, Vec3, Point, NVec3, reflect};
use crate::shared::{Ray, Random, random_unit_sphere, lerp};


pub enum MaterialType {
    Diffuse(Vec3),
    Metal(Vec3, f32),
}

trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)>;
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)> {
        match self {
            MaterialType::Diffuse(color)     => diffuse_scatter(*color, ray, hit, random),
            MaterialType::Metal(color, fuzz) => metal_scatter(*color, *fuzz, ray, hit, random),
        }
    }
}

fn diffuse_scatter(color: Vec3, _ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)> {
    let scatter = hit.normal + random_unit_sphere(random);

    // Catch degenerate scatter direction
    if scatter.near_zero() {
        Some((color, Ray{origin: hit.position, direction: hit.normal}))
    } else {
        Some((color, Ray{origin: hit.position, direction: scatter.normalize()}))
    }
}

fn metal_scatter(color: Vec3, fuzz: f32, ray: &Ray, hit: &HitRecord, random: &mut Random) -> Option<(Vec3, Ray)> {
    let reflected = reflect(&ray.direction, &hit.normal);
    let direction = reflected + fuzz*random_unit_sphere(random);

    if direction.dot(&hit.normal) > 0.0 {
        Some((color, Ray{ origin: hit.position, direction: direction.normalize() }))
    } else {
        None
    }
}


// ----------------- HITTABLES ----------------------
struct HitRecord<'a> {
    position: Point,
    normal: NVec3,
    t: f32,
    material: &'a MaterialType,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: MaterialType,
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

        return Some(HitRecord{ t, position, normal, material: &self.material });
    }
}


pub struct World {
    pub spheres: Vec<Sphere>,
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


pub fn ray_color(ray: &Ray, world: &World, random: &mut Random, depth: i32) -> Vec3 {
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


pub const RED_DIFFUSE   : MaterialType = MaterialType::Diffuse{ 0: Vec3 { x: 1.0, y: 0.0, z: 0.0 } };
pub const GREEN_DIFFUSE : MaterialType = MaterialType::Diffuse{ 0: Vec3 { x: 0.0, y: 1.0, z: 0.0 } };
pub const BLUE_DIFFUSE  : MaterialType = MaterialType::Diffuse{ 0: Vec3 { x: 0.0, y: 0.0, z: 1.0 } };

pub const GROUND_MATERIAL : MaterialType = MaterialType::Diffuse{ 0: Vec3 { x: 0.8, y: 0.8, z: 0.0 } };
pub const BALL_MATERIAL   : MaterialType = MaterialType::Diffuse{ 0: Vec3 { x: 0.7, y: 0.3, z: 0.3 } };

pub const METAL_MATERIAL_1 : MaterialType = MaterialType::Metal{ 0: Vec3 { x: 0.8, y: 0.8, z: 0.8 }, 1: 0.3 };
pub const METAL_MATERIAL_2 : MaterialType = MaterialType::Metal{ 0: Vec3 { x: 0.8, y: 0.6, z: 0.2 }, 1: 1.0 };
