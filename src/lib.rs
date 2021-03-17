#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod maths;
pub mod parser;
pub mod camera;
pub mod image;
pub mod random;
pub mod mat3;
pub mod materials;
pub mod common;


use materials::{MaterialType, Material, ScatterData};
use mat3::Mat3;
use random::Random;
use image::{Framebuffer, write_image};
use camera::{Camera, Radians};
use maths::{Vec3, Point, NVec3, reflect, refract, IVector, Y_AXIS};
use common::{World, Options, ray_trace};



use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::path::Path;
use std::panic::catch_unwind;

struct OpaqueData<'a> {
    camera: Camera,
    world : World<'a>,
}

#[repr(C)]
pub struct CFramebuffer {
    pub max_color_value: usize,
    pub width:  usize,
    pub height: usize,
    pub pixels: std::ptr::NonNull<Vec3>,
}

#[repr(C)]
pub struct Data<'a> {
    opaque: std::ptr::NonNull<OpaqueData<'a>>,
    pub framebuffer: CFramebuffer,
}

#[repr(C)]
pub struct Array<T> {
    count: usize,
    data:  std::ptr::NonNull<T>
}


#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
pub struct Bitmap{
    width:  usize,
    pixels: Array<Color>,
}


#[no_mangle]
pub extern "C" fn create_bitmap(width: usize, height: usize, source: *const c_char) -> Bitmap {
    let options = Options{ samples_per_pixel: 50, max_ray_bounces: 8 };
    let c_str = unsafe { CStr::from_ptr(source) };
    let (_camera, spheres) = parser::parse_input(c_str.to_str().unwrap()).unwrap();
    let world = World::new(spheres, vec![]);

    let aspect_ratio = width / height;

    let camera = camera::Camera::new_look_at(
        Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Y_AXIS.into(), Radians(std::f32::consts::PI / 2.0), aspect_ratio as f32
    );

    let framebuffer = Framebuffer::new(width, height);
    let framebuffer = ray_trace(world, camera, framebuffer, &options);

    let mut pixels = Vec::with_capacity(framebuffer.width * framebuffer.height);
    for row in (0usize..framebuffer.height).rev() {
        for column in 0usize..framebuffer.width {
            let color = framebuffer[[row, column]];
            pixels.push(
                Color{
                    r: color.x as u8,
                    g: color.y as u8,
                    b: color.z as u8,
                    a: 255
                }
            )
        }
    }

    let count = width * height;
    let array = Array {
        count, data: unsafe { std::ptr::NonNull::new_unchecked(pixels.as_mut_ptr()) }
    };
    let bitmap = Bitmap {
        width, pixels: array
    };
    std::mem::forget(pixels);

    bitmap
}


// #[no_mangle]
// pub extern "C" fn load() -> *mut c_void {
//     Box::into_raw(Box::new(Foo::new())) as *mut _
// }


// #[no_mangle]
// pub extern "C" fn loading<'a>(file: *const c_char, width: u32, height: u32) -> *mut Data<'a> {
//     // catch_unwind(|| {
//     //     panic!("Oops!");
//     // });
//     let path = unsafe { CStr::from_ptr(file).to_str().unwrap() };
//
//     let result = parser::parse_input(
//         &std::fs::read_to_string(path)
//             .map_err(|_| parser::ParseError::WrongSyntax).unwrap()
//     );
//
//     let (camera, spheres) = result.unwrap();
//     let world = World::new(spheres, vec![]);
//     let mut framebuffer = Framebuffer::new(width as usize, height as usize);
//
//     let opaque = Box::new(OpaqueData{ camera, world });
//
//     let data = Data {
//         opaque: unsafe { std::ptr::NonNull::new_unchecked(Box::into_raw(opaque)) },
//         framebuffer: CFramebuffer {
//             max_color_value: framebuffer.max_color_value,
//             width: framebuffer.width,
//             height: framebuffer.height,
//             pixels: unsafe { std::ptr::NonNull::new_unchecked(framebuffer.pixels.as_mut_ptr()) }
//         }
//     };
//
//     std::mem::forget(framebuffer);
//     Box::into_raw(Box::new(data)) as *mut Data
// }





use std::ffi::CString;

#[no_mangle]
pub extern fn rust_hello(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_hello_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
