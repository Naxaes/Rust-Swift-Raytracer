use std::collections::HashMap;
use std::error::Error;

use crate::{MaterialType, Camera, Sphere};
use crate::maths::{Vec3, Point};


pub fn parse_world() -> Option<(Camera, Vec<Sphere>)>  {
    parse_input(&std::fs::read_to_string("src/world.txt").ok()?)
}


pub fn skip_whitespace(source: &str) -> &str {
    let index = source.find(|c: char| !c.is_whitespace()).unwrap_or(source.len());
    &source[index..]
}

pub fn get_identifier(source: &str) -> (&str, &str) {
    let index = source.find(|c: char| !(c.is_alphanumeric() || c == '_')).unwrap_or(source.len());
    (&source[index..], &source[..index])  // TODO: Index error on first slice.
}

/// Checks if `source` starts with `target` and returns a
/// slice of `source` from after the `target`.
pub fn starts_with<'a>(source: &'a str, target: &str) -> Option<&'a str> {
    let size = target.len();
    if source.len() >= size && source[0..size] == target[..] {
        Some(&source[size..])
    } else {
        None
    }
}

pub fn parse_int(source: &str) -> Option<(&str, i32)> {
    let data = source.as_bytes();
    let mut index = 0;

    for c in data[index..data.len()].into_iter() {
        if b'0' <= *c && *c <= b'9' {
            index += 1;
        } else {
            break;
        }
    }

    let result = if let Ok(a) = source[0..index].parse::<i32>() { a } else { return None };
    Some((&source[index..], result))  // Index error
}


pub fn parse_float(source: &str) -> Option<(&str, f32)> {
    let data = source.as_bytes();
    let mut found_dot = false;
    let mut index     = 0;

    if data.len() < 3 {
        return None;
    }

    if data[0] == b'-' {
        index = 1;
    }

    for c in data[index..data.len()].into_iter() {
        if b'0' <= *c && *c <= b'9' {
            index += 1;
        } else if *c == b'.' {
            if !found_dot { found_dot = true } else { return None }
            index += 1;
        } else {
            break;
        }
    }

    let result = if let Ok(a) = source[0..index].parse::<f32>() { a } else { return None };
    Some((&source[index..], result))  // Index error
}

pub fn parse_vec3(source: &str) -> Option<(&str, Vec3)> {
    let (source, x) = parse_float(source)?;
    let source      = skip_whitespace(source);
    let (source, y) = parse_float(source)?;
    let source      = skip_whitespace(source);
    let (source, z) = parse_float(source)?;
    Some((source, Vec3{x, y, z}))
}

/// camera : camera origin <f32> <f32> <f32> aspect <f32> ;
pub fn parse_camera(source: &str) -> Option<(&str, Camera)> {
    if let Some(source) = starts_with(source, "camera") {
        let source = skip_whitespace(source);

        let source = starts_with(source, "origin")?;
        let source = skip_whitespace(source);
        let (source, o) = parse_vec3(source)?;
        let source = skip_whitespace(source);

        let source = starts_with(source, "aspect")?;
        let source = skip_whitespace(source);
        let (source, a) = parse_float(source)?;
        let source = skip_whitespace(source);

        let source = starts_with(source, ";")?;

        return Some((
            source, Camera::at(o, a)
        ));
    }
    None
}


/// material :  material <name> : <type> ;
/// type     :  <diffuse> | <metal>
/// diffuse  :  Diffuse color <f32> <f32> <f32>
/// metal    :  Metal color <f32> <f32> <f32> fuzz <f32>
pub fn parse_material(source: &str) -> Option<(&str, &str, MaterialType)> {
    if let Some(source) = starts_with(source, "material") {
        let source = skip_whitespace(source);

        let (source, name) = get_identifier(source);
        let source = skip_whitespace(source);
        let source = starts_with(source, ":")?;
        let source = skip_whitespace(source);

        if let Some(source) = starts_with(source, "Diffuse") {
            let source = skip_whitespace(source);

            let source = starts_with(source, "color")?;
            let source = skip_whitespace(source);
            let (source, c) = parse_vec3(source)?;
            let source = skip_whitespace(source);

            let source = starts_with(source, ";")?;

            return Some((source, name, MaterialType::Diffuse(c)));
        }

        if let Some(source) = starts_with(source, "Metal") {
            let source = skip_whitespace(source);

            let source = starts_with(source, "color")?;
            let source = skip_whitespace(source);
            let (source, c) = parse_vec3(source)?;
            let source = skip_whitespace(source);

            let source = starts_with(source, "fuzz")?;
            let source = skip_whitespace(source);
            let (source, f) = parse_float(source)?;
            let source = skip_whitespace(source);

            let source = starts_with(source, ";")?;

            return Some((source, name, MaterialType::Metal(c, f)));
        }
    }
    None
}

/// sphere  : sphere center <f32> <f32> <f32> radius <f32> material <name> ;
pub fn parse_sphere<'a>(source: &'a str, materials: &HashMap<&'a str, MaterialType>) -> Option<(&'a str, Sphere)> {
    if let Some(source) = starts_with(source, "sphere") {
        let source = skip_whitespace(source);

        let source = starts_with(source, "center")?;
        let source = skip_whitespace(source);
        let (source, c) = parse_vec3(source)?;
        let source = skip_whitespace(source);

        let source = starts_with(source, "radius")?;
        let source = skip_whitespace(source);
        let (source, r) = parse_float(source)?;
        let source = skip_whitespace(source);

        let source = starts_with(source, "material")?;
        let source = skip_whitespace(source);
        let (source, m) = get_identifier(source);
        let source = skip_whitespace(source);

        let source = starts_with(source, ";")?;

        return Some((
            source, Sphere{ center: c, radius: r, material: materials.get(m)?.to_owned() }
        ));
    }

    None
}


/// --- Syntax ----
/// program  :  <camera> (<material>)* (<sphere>)*
/// camera   :  camera origin <f32> <f32> <f32> aspect <f32> ;
/// material :  material <name> : <type> ;
/// type     :  <diffuse> | <metal>
/// diffuse  :  Diffuse color <f32> <f32> <f32>
/// metal    :  Metal color <f32> <f32> <f32> fuzz <f32>
/// sphere   :  center <f32> <f32> <f32> radius <f32> material <name> ;
pub fn parse_input(mut source: &str) -> Option<(Camera, Vec<Sphere>)> {
    let mut materials = HashMap::new();
    let mut spheres : Vec<Sphere> = Vec::new();

    // Parse camera
    let camera =
        if let Some((next, camera)) = parse_camera(source) {
            source = skip_whitespace(next);
            camera
        } else {
            return None;
        };

    // Parse all materials
    while let Some((next, name, material)) = parse_material(source) {
        materials.insert(name, material);
        source = skip_whitespace(next);
    }

    // Parse all spheres.
    while let Some((next, sphere)) = parse_sphere(source, &materials) {
        spheres.push(sphere);
        source = skip_whitespace(next);
    }

    if !source.is_empty() {
        None
    } else {
        Some((camera, spheres))
    }
}