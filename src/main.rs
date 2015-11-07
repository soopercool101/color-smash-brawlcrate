extern crate image;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

use image::{
    GenericImage,
    Pixel,
    Rgba,
};

fn main() {
    let image = image::open(&Path::new("00.png")).unwrap();

    let colors = image.pixels().map(|(_, _, color)| color);

    let quantization_map: HashMap<Color, Color> = quantize(colors);
}

trait ByteUtils {
    fn convert_8_bits_to_5(self) -> u8;
    fn convert_8_bits_to_4(self) -> u8;
    fn convert_8_bits_to_3(self) -> u8;

    fn convert_5_bits_to_8(self) -> u8;
    fn convert_4_bits_to_8(self) -> u8;
    fn convert_3_bits_to_8(self) -> u8;

    fn approximate_5_bits(self) -> u8;
    fn approximate_4_bits(self) -> u8;
    fn approximate_3_bits(self) -> u8;
}

impl ByteUtils for u8 {
    fn convert_8_bits_to_5(self) -> u8 {
        (((self as u32) * 31 + 127) / 255) as u8
    }
    fn convert_8_bits_to_4(self) -> u8 {
        (((self as u32) + 8) / 17) as u8
    }
    fn convert_8_bits_to_3(self) -> u8 {
        (((self as u32) * 7 + 127) / 255) as u8
    }

    fn convert_5_bits_to_8(self) -> u8 {
        (((self as u32) * 255 + 15) / 31) as u8
    }
    fn convert_4_bits_to_8(self) -> u8 {
        ((self as u32) * 17) as u8
    }
    fn convert_3_bits_to_8(self) -> u8 {
        (((self as u32) * 255 + 3) / 7) as u8
    }

    fn approximate_5_bits(self) -> u8 {
        self.convert_8_bits_to_5().convert_5_bits_to_8()
    }
    fn approximate_4_bits(self) -> u8 {
        self.convert_8_bits_to_4().convert_4_bits_to_8()
    }
    fn approximate_3_bits(self) -> u8 {
        self.convert_8_bits_to_3().convert_3_bits_to_8()
    }
}

type Color = Rgba<u8>;

trait ColorUtils {
    fn as_rgb5a3(self) -> Color;
}

impl ColorUtils for Color {
    fn as_rgb5a3(self) -> Color {
        let (r, g, b, a) = self.channels4();
        let new_a = a.approximate_3_bits();
        if new_a == 0xFF {
            Color { data: [r.approximate_5_bits(), g.approximate_5_bits(), b.approximate_5_bits(), new_a] }
        } else {
            Color { data: [r.approximate_4_bits(), g.approximate_4_bits(), b.approximate_4_bits(), new_a] }
        }
    }
}

trait Data : Eq + Hash {
    fn distance_to(&self, other: &Self) -> u64;
}

impl Data for Color {
    fn distance_to(&self, other: &Color) -> u64 {
        let (r1, g1, b1, a1) = self.channels4();
        let (r2, g2, b2, a2) = other.channels4();

        let opaque_distance = ((r1 as i32) - (r2 as i32)).pow(2) +
                              ((g1 as i32) - (g2 as i32)).pow(2) +
                              ((b1 as i32) - (b2 as i32)).pow(2);

        let alpha_distance = ((a1 as i32) - (a2 as i32)).pow(2) * 3;

        ((opaque_distance as u64) * (a1 as u64) * (a2 as u64)) + ((alpha_distance as u64) * 255 * 255)
    }
}

fn quantize<I>(items: I) -> HashMap<I::Item, I::Item>
    where I: Iterator,
          I::Item: Data {

    let mut count_of_items: HashMap<I::Item, u32> = HashMap::new();

    for item in items {
        let counter = count_of_items.entry(item).or_insert(0);
        *counter += 1;
    }

    count_of_items.shrink_to_fit();

    println!("{:?}", count_of_items.len());

    let mut quantization_map = HashMap::new();
    quantization_map
}