use image::{self, DynamicImage, ImageFormat};
use std::io::Cursor;

pub fn get_cropped_image(url: String, width: u32, height: u32) -> Vec<u8> {
    let image_bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let image = image::load_from_memory(&image_bytes).unwrap();
    let cropped = center_crop_image(image, width, height);
    get_buffer(&cropped)
}

fn center_crop_image(mut image: DynamicImage, width: u32, height: u32) -> DynamicImage {
    let x = image.width() / 2 - width / 2;
    let y = image.height() / 2 - height / 2;
    image.crop(x, y, width, height)
}

fn get_buffer(image: &DynamicImage) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, ImageFormat::Png).unwrap();

    buffer.into_inner()
}
