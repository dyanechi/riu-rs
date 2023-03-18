use std::{path::Path, fs::create_dir};

use image::{GenericImageView, RgbImage, ImageBuffer, imageops};

const IN_PATH: &'static str = "./input";
const OUT_PATH: &'static str = "./output";

pub struct ImgGen {}
impl ImgGen {
    pub fn new() -> Self {
        if ! Path::new(IN_PATH).exists() {
            create_dir(IN_PATH).unwrap();
        }
        if ! Path::new(OUT_PATH).exists() {
            create_dir(OUT_PATH).unwrap();
        }

        ImgGen {}
    }

    pub fn upscale_times(&self, filename: &str, times: u8) -> RgbImage {
        let mut img = image::open(format!("{}/{}", IN_PATH, filename)).unwrap().to_rgb8();
        for _ in 0..times {
            img = self.upscale(img);
        }
        img
    }

    pub fn downscale_times(&self, filename: &str, times: u8) -> RgbImage {
        let mut img = image::open(format!("{}/{}", IN_PATH, filename)).unwrap().to_rgb8();
        for _ in 0..times {
            img = self.downscale(img);
        }
        img
    }

    pub fn upscale(&self, img: RgbImage) -> RgbImage {
        let (w, h) = img.dimensions();

        let img: RgbImage = ImageBuffer::from_fn(w * 2, h, |x, y| {
            if x % 2 == 1 && x < w * 2 - 1 {
                let left = img.get_pixel(x / 2, y).0;
                let right = img.get_pixel(x / 2 + 1, y).0;
                let dot = gen_pixel(left, right);
                image::Rgb(dot)
            } else {
                image::Rgb(img.get_pixel(x / 2, y).0)
            }
        });

        let (w, h) = img.dimensions();
        let img: RgbImage = ImageBuffer::from_fn(w, h * 2, |x, y| {
            if y % 2 == 1 && y < h * 2 - 1  {
                let left = img.get_pixel(x, y / 2).0;
                let right = img.get_pixel(x, y / 2 + 1).0;
                let dot = gen_pixel(left, right);
                image::Rgb(dot)
            } else {
                image::Rgb(img.get_pixel(x, y / 2).0)
            }
        });

        img
    }

    pub fn downscale(&self, img: RgbImage) -> RgbImage {
        let (w, h) = img.dimensions();

        let mut new_img: RgbImage = ImageBuffer::new(w/2, h/2);

        for y in 1..w {
            if y % 2 == 1 { continue; }
            for x in 1..h {
                if x % 2 == 1 { continue; }
            
                let x0x1y0 = gen_pixel(img.get_pixel(x-1, y-1).0, img.get_pixel(x, y-1).0);
                let x0x1y1 = gen_pixel(img.get_pixel(x-1, y).0, img.get_pixel(x, y).0);
                let dot = gen_pixel(x0x1y0, x0x1y1);
                new_img.put_pixel(x/2-1, y/2-1, image::Rgb(dot));
            }
        }

        new_img
    } 


    pub fn save(&self, img: RgbImage, filename: &str) {
        let filename = format!("out_1{}", filename);
        img.save(format!("{}/{}", OUT_PATH, filename)).unwrap();
    }
}

pub fn gen_pixel(left: [u8; 3], right: [u8; 3]) -> [u8; 3] {
    let dot = [
        norm_u8(left[0], right[0]),
        norm_u8(left[1], right[1]),
        norm_u8(left[2], right[2]),
    ];
    dot
}

pub fn norm_u8(left: u8, right: u8) -> u8 {
    // let dot = left.abs_diff(right);

    // Calculate AVG between colors
    let avg = ([
        left as u16,
        right as u16
    ].iter().sum::<u16>() / 2) as u8;

    avg
}

pub fn lerp_px(left: [u8; 3], right: [u8; 3], t: f32) -> [u8; 3] {
    let px = [0, 0, 0];

    [0, 0, 0]
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up() {
        let img_gen = ImgGen::new();

        let new_img = img_gen.upscale_times("anime-img1.jpg", 1);
    }
}