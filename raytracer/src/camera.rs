use crate::common::Ray;
use crate::maths::{Point, Vec3, IVector, NVec3};


pub struct Radians(pub f32);


pub struct Camera {
    origin: Point,

    // 3 points for a plane.
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical:   Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self::new_at(Point::new(0.0, 0.0, 0.0), aspect_ratio)
    }
    pub fn new_at(origin: Point, aspect_ratio: f32) -> Self {
        // Camera
        let viewport_height = 2.0;
        let viewport_width  = aspect_ratio * viewport_height;
        let focal_length    = 1.0;

        // Viewport
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical   = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - Vec3::new(viewport_width / 2.0, viewport_height / 2.0, focal_length);

        Camera { origin, lower_left_corner, horizontal, vertical }
    }
    pub fn new_with_vertical_fov(origin: Point, vertical_fov: Radians, aspect_ratio: f32) -> Self {
        let h = f32::tan(vertical_fov.0 / 2.0);

        // Camera
        let viewport_height = 2.0 * h;
        let viewport_width  = aspect_ratio * viewport_height;
        let focal_length    = 1.0;

        // Viewport
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical   = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - Vec3::new(viewport_width / 2.0, viewport_height / 2.0, focal_length);

        Camera { origin, lower_left_corner, horizontal, vertical }
    }
    pub fn new_look_at(origin: Point, look_at: Point, up: NVec3, vertical_fov: Radians, aspect_ratio: f32) -> Self {
        assert!(!(origin-look_at).near_zero(), "Origin and look_at must differ!");

        // Camera
        let viewport_height = 2.0 * f32::tan(vertical_fov.0 / 2.0);
        let viewport_width  = viewport_height * aspect_ratio;

        // Local coordinate system
        let w = (origin - look_at).normalize();
        let u = up.cross(&w);
        let v = w.cross(&u);

        assert!(v.y().abs() > 1e-8, "Origin and look_at can't have the same z-coordinate.");

        // Viewport
        let horizontal = u * viewport_width;
        let vertical   = v * viewport_height;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

        Camera { origin, lower_left_corner, horizontal, vertical }
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.horizontal.x / self.vertical.y
    }

    // NOTE(ted): (u, v) is only NSC if there are axis aligned vectors. When having
    //  a plane that isn't axis-aligned, (u, v) will not correspond to the NSC.
    // /// Cast a ray from the normalized screen coordinates u and v.
    // pub fn cast_ray(&self, u: f32, v: f32) -> Ray {
    //     Ray {
    //         origin: self.origin,
    //         direction: (self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin).normalize()
    //     }
    // }
    /// Cast a ray from the normalized viewport coordinates s and t.
    pub fn cast_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            (self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin).normalize()
        )
    }

    pub fn position(&self) -> Vec3 {
        self.origin
    }
}
