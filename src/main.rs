use image::io::Reader;
use image::DynamicImage;
use image::GenericImageView;
use image::Rgba;

const ASCII_CHAR: [char; 12] = ['█', '▓', '@', '&', '%', '#', '*', '+', '-', ',', '.', ' '];

fn main() {
    let img = transform_image("/home/jchhabra/rust-projects/terminartor/pexels-olly-733872.jpg");

    let scale = 8;
    let mut ascii_art = String::new();
    for j in 0..img.height() {
        for i in 0..img.width() {
            if j % (scale * 2) == 0 && i % scale == 0 {
                let pixel = img.get_pixel(i, j);
                let intensity = get_pixel_intensity(pixel);
                ascii_art.push(get_intensive_ascii_char(intensity))
            }
        }

        if j % (scale * 2) == 0 {
            ascii_art.push('\n');
        }
    }

    print!("{}", ascii_art);
}

fn get_pixel_intensity(pixel: Rgba<u8>) -> u8 {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;

    let intensity = ((r + g + b) / (3 as f32)).floor() as u8;

    intensity
}

fn get_intensive_ascii_char(intensity: u8) -> char {
    let mut index: usize = ASCII_CHAR.len() - ((intensity / 21) as f32).floor() as usize;

    if index == 12 {
        index -= 1;
    }

    ASCII_CHAR[index]
}

fn transform_image(path: &str) -> DynamicImage {
    let img = Reader::open(path)
        .unwrap()
        .decode()
        .expect("Loading image failed");

    let img = img.grayscale();

    img
}
