use std::fs::File;
use std::io::{stdout, Write, Result};
use std::path::Path;

use crate::maths::Vec3;


#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub max_color_value: usize,
    pub width:  usize,
    pub height: usize,
    pub pixels: Vec<Vec3>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
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



/// https://en.wikipedia.org/wiki/Netpbm#PPM_example
///
/// The image format:
///
///   P3
///   3 2
///   255
///   # The part above is the header
///   # "P3" means this is a RGB color image in ASCII
///   # "3 2" is the width and height of the image in pixels
///   # "255" is the maximum value for each color
///   # The part below is image data: RGB triplets
///   255   0   0  # red
///   0 255   0  # green
///   0   0 255  # blue
///   255 255   0  # yellow
///   255 255 255  # white
///   0   0   0  # black
///
pub fn write_image(framebuffer: &Framebuffer, output: Option<&str>) -> Result<()> {
    let mut writer = match output {
        Some(x) => {
            Box::new(File::create(&Path::new(x)).unwrap()) as Box<dyn Write>
        }
        None => Box::new(stdout()) as Box<dyn Write>,
    };


    write!(&mut writer,
        "P3\n{width} {height}\n{max_color_value}\n",
        width=framebuffer.width, height=framebuffer.height, max_color_value=framebuffer.max_color_value
    )?;

    for row in (0usize..framebuffer.height).rev() {
        for column in 0usize..framebuffer.width {
            let color = framebuffer[[row, column]];
            write!(&mut writer, "{} {} {}\n", color.x as u32, color.y as u32, color.z as u32)?;
        }
    }

    Ok(())
}
