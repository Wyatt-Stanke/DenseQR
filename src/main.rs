use image::{ImageBuffer, RgbImage};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    println!("Hello, world!");
    make_qr(&[1u8]);
}

#[derive(Debug, EnumIter)]
enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
}

impl Into<RGB8Color> for Color {
    fn into(self) -> RGB8Color {
        match self {
            Color::Black => RGB8Color::new(0, 0, 0),
            Color::White => RGB8Color::new(255, 255, 255),
            Color::Red => RGB8Color::new(255, 0, 0),
            Color::Green => RGB8Color::new(0, 255, 0),
            Color::Blue => RGB8Color::new(0, 0, 255),
            Color::Yellow => RGB8Color::new(255, 255, 0),
            Color::Orange => RGB8Color::new(255, (256 / 2) as u8, 0),
            Color::Purple => RGB8Color::new((256 / 2) as u8, 0, 255),
        }
    }
}

#[derive(Debug)]
struct RGB8Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGB8Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        RGB8Color {
            red: r,
            green: g,
            blue: b,
        }
    }
}

impl Into<[u8; 3]> for RGB8Color {
    fn into(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

fn make_qr(data: &[u8]) {
    // Generate image buffer
    let mut buffer: RgbImage = ImageBuffer::new(256, 256);

    // Generate correction block
    let mut correction_block: Vec<RGB8Color> = vec![];
    for color in Color::iter() {
        correction_block.push(color.into());
    }

    println!("{:?}", correction_block);

    // Add correction block
    buffer.get_pixel_mut(0, 0).0 = correction_block[0].into();
    buffer.get_pixel_mut(1, 0).0 = correction_block[1].into();
    buffer.get_pixel_mut(2, 0).0 = correction_block[2].into();
    buffer.get_pixel_mut(2, 1).0 = correction_block[3].into();
    buffer.get_pixel_mut(2, 2).0 = correction_block[4].into();
}
