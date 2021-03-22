use std::num::NonZeroU32;

pub mod dynamic_dispatch;
pub mod enum_dispatch;
pub mod shared;
pub mod maths;

use dynamic_dispatch as dd;
use enum_dispatch as ed;

use crate::maths::{Point, Vector};
use crate::shared::{Random, Camera};

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};


impl std::fmt::Display for maths::NVec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x(), self.y(), self.z())
    }
}


fn bench_tracer(c: &mut Criterion) {
    let world_dynamic = dd::World {
        spheres: vec![
            dd::Sphere{ center: Point{ x:  0.0, y: -100.5, z: -1.0 }, radius: 100.0, material: Box::new(dd::GROUND_MATERIAL)},
            dd::Sphere{ center: Point{ x:  0.0, y:  0.0,   z: -1.0 }, radius: 0.5,   material: Box::new(dd::BALL_MATERIAL)},
            dd::Sphere{ center: Point{ x: -1.0, y:  0.0,   z: -1.0 }, radius: 0.5,   material: Box::new(dd::METAL_MATERIAL_1)},
            dd::Sphere{ center: Point{ x:  1.0, y:  0.0,   z: -1.0 }, radius: 0.5,   material: Box::new(dd::METAL_MATERIAL_2)},
        ]
    };

    let world_enum = ed::World {
        spheres: vec![
            ed::Sphere{ center: Point{ x:  0.0, y: -100.5, z: -1.0 }, radius: 100.0, material: ed::GROUND_MATERIAL},
            ed::Sphere{ center: Point{ x:  0.0, y:  0.0,   z: -1.0 }, radius: 0.5,   material: ed::BALL_MATERIAL},
            ed::Sphere{ center: Point{ x: -1.0, y:  0.0,   z: -1.0 }, radius: 0.5,   material: ed::METAL_MATERIAL_1},
            ed::Sphere{ center: Point{ x:  1.0, y:  0.0,   z: -1.0 }, radius: 0.5,   material: ed::METAL_MATERIAL_2},
        ]
    };

    let camera = Camera::new(16.0 / 9.0);

    let parameters = [
        (0.1f32, 0.1f32, 245),
        (0.1f32, 0.3f32, 245),
        (0.1f32, 0.6f32, 245),
        (0.1f32, 0.9f32, 245),

        (0.3f32, 0.1f32, 245),
        (0.3f32, 0.3f32, 245),
        (0.3f32, 0.6f32, 245),
        (0.3f32, 0.9f32, 245),

        (0.6f32, 0.1f32, 245),
        (0.6f32, 0.3f32, 245),
        (0.6f32, 0.6f32, 245),
        (0.6f32, 0.9f32, 245),

        (0.9f32, 0.1f32, 245),
        (0.9f32, 0.3f32, 245),
        (0.9f32, 0.6f32, 245),
        (0.9f32, 0.9f32, 245),
    ];

    let mut group = c.benchmark_group("Raytracer");
    for (u, v, i) in parameters.iter() {
        let ray = camera.cast_ray(*u, *v);

        group.bench_with_input(
            BenchmarkId::new("Dynamic", &ray.direction),
            &(&ray, &world_dynamic, *i),
            |b, (ray, world, i)|
                b.iter(|| dd::ray_color(ray, world, &mut Random::new(NonZeroU32::new(*i).unwrap()), 8))
        );

        group.bench_with_input(
            BenchmarkId::new("Enum", &ray.direction),
            &(&ray, &world_enum, *i),
            |b, (ray, world, i)|
                b.iter(|| ed::ray_color(ray, world, &mut Random::new(NonZeroU32::new(*i).unwrap()), 8))
        );
    }
    group.finish();
}

criterion_group!(benches, bench_tracer);
criterion_main!(benches);