use crate::common::{HitRecord, Ray, random_unit_sphere};
use crate::random::{Random};
use crate::maths::{Vec3, NVec3, reflect, refract, IVector};

#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    Diffuse(Vec3),
    Metal(Vec3, f32),  // TODO: Encode the fuzz in the length of the vector.
    Dielectric(f32),
    Emission(Vec3),
}

pub struct ScatterData {
    pub color:    Vec3,
    pub next_ray: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, random: &mut Random) -> ScatterData;
}




fn hit_front_face(direction: &Vec3, normal: &NVec3) -> bool {
    direction.dot(normal) >= 0.0
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, random: &mut Random) -> ScatterData {
        // TODO: Return None on black colors?
        match self {
            MaterialType::Diffuse(color)     => diffuse_scatter(*color, ray, hit, random),
            MaterialType::Metal(color, fuzz) => metal_scatter(*color, *fuzz, ray, hit, random),
            MaterialType::Dielectric(ir)     => dielectric_scatter(*ir, ray, hit, random),
            MaterialType::Emission(color)    => emission_scatter(*color, ray, hit, random),
        }
    }
}

fn diffuse_scatter(color: Vec3, _ray: &Ray, hit: &HitRecord, random: &mut Random) -> ScatterData {
    // return ScatterData { color, next_ray: None };
    let scatter = hit.normal + random_unit_sphere(random);

    // Catch degenerate scatter direction
    if scatter.near_zero() {
        ScatterData{ color, next_ray: Some(Ray::new(hit.position, hit.normal)) }
    } else {
        ScatterData{ color, next_ray: Some(Ray::new(hit.position, scatter.normalize())) }
    }
}

fn metal_scatter(color: Vec3, fuzz: f32, ray: &Ray, hit: &HitRecord, random: &mut Random) -> ScatterData {
    let reflected = reflect(ray.direction.into(), hit.normal);
    let direction = reflected + fuzz*random_unit_sphere(random);

    if hit_front_face(&direction, &hit.normal) {
        ScatterData { color, next_ray: Some(Ray::new(hit.position, direction.normalize())) }
    } else {
        ScatterData { color, next_ray: None }
    }
}

fn dielectric_scatter(ir: f32, ray: &Ray, hit: &HitRecord, _random: &mut Random) -> ScatterData {
    let (normal, refraction_ratio) =
        if hit_front_face(&ray.direction.into(), &hit.normal) {
            (-hit.normal, 1.0/ir)  // Ray is inside the object.
        } else {
            (hit.normal, ir)       // Ray is outside the object.
        };


    // fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
    //     // Use Schlick's approximation for reflectance.
    //     let r0 = (1.0-refraction_ratio) / (1.0+refraction_ratio);
    //     let r0 = r0*r0;
    //     r0 + (1.0-r0) * f32::powi(1.0 - cos_theta, 5)
    // }
    // let cos_theta = NVec3::dot(&-ray.direction, &hit.normal);
    // let sin_theta = f32::sqrt(1.0 - cos_theta*cos_theta);
    //
    // let cannot_refract = refraction_ratio * sin_theta < 1.0;
    // let direction =
    //     if cannot_refract || reflectance(cos_theta, refraction_ratio) > random.random_f32() {
    //         reflect(ray.direction.into(), hit.normal).normalize()
    //     } else {
    //         refract(ray.direction, hit.normal, refraction_ratio).normalize()
    //     };
    //
    // let scattered = Ray::new(hit.position, direction);
    // ScatterData { color: Vec3::new(1.0, 1.0, 1.0), next_ray: Some(scattered) }

    let refracted = refract(ray.direction, normal, refraction_ratio);
    let scattered = Ray::new(hit.position, refracted.normalize());
    ScatterData { color: Vec3::new(1.0, 1.0, 1.0), next_ray: Some(scattered) }
}


fn emission_scatter(color: Vec3, _ray: &Ray, _hit: &HitRecord, _random: &mut Random) -> ScatterData {
    ScatterData { color, next_ray: None }
}

