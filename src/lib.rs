use image::{DynamicImage, GenericImageView, ImageError, ImageReader, Rgba};
use std::{
    error::Error,
    fs::File,
    io::{self, Write},
};

pub struct Config {
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 || args.len() < 2 {
            return Err("Incorrect number of arguments");
        }
        let file_path = args[1].clone();

        Ok(Config {
            file_path: file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let img = read_image(&config.file_path).unwrap();
    print_photo(img);

    Ok(())
}

pub fn read_image(path: &str) -> Result<DynamicImage, ImageError> {
    let img = ImageReader::open(path)?.decode()?;

    Ok(img)
}

pub fn print_photo(img: DynamicImage) {
    for row in 0..img.height() {
        for pixel in 0..img.width() {
            let out = bright_to_ascii(img.get_pixel(pixel, row));
            print!("{}", out);
        }
        print!("\n");
    }
}

pub fn write_to_file(img: DynamicImage) {
    let file = File::create("hello.txt").expect("something wrong with creating file");
    let mut ascii_img = String::new();
    let mut fw = io::BufWriter::new(file);

    for row in 0..img.height() {
        for pixel in 0..img.width() {
            let out = bright_to_ascii(img.get_pixel(pixel, row));
            ascii_img.push(out);
        }
        ascii_img.push('\n');
    }

    fw.write_all(ascii_img.as_bytes())
        .expect("write to file failed");
}

pub fn bright_to_ascii(pixel: Rgba<u8>) -> char {
    let brightness =
        0.299 * (pixel.0[0] as f64) + 0.587 * (pixel.0[1] as f64) + 0.114 * (pixel.0[2] as f64);

    if brightness <= 63.000 {
        'a'
    } else if brightness > 64.000 && brightness <= 128.000 {
        '_'
    } else if brightness > 128.000 && brightness <= 192.000 {
        '$'
    } else {
        'D'
    }
}

#[cfg(test)]
mod test {
    use image::Rgba;

    use super::*;

    #[test]
    fn correct_ascii() {
        let pixel = Rgba([255, 255, 255, 255]);
        assert_eq!(bright_to_ascii(pixel), 'D');
    }

    #[test]
    fn test_write_to_file() {
        use image::{DynamicImage, RgbaImage};
        use std::fs;

        // 1. Make a 2x2 image with known colors
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, image::Rgba([0, 0, 0, 255])); // Black
        img.put_pixel(1, 0, image::Rgba([255, 255, 255, 255])); // White
        img.put_pixel(0, 1, image::Rgba([127, 127, 127, 255])); // Gray
        img.put_pixel(1, 1, image::Rgba([200, 200, 200, 255])); // Light Gray

        let dyn_img = DynamicImage::ImageRgba8(img);

        // 2. Call your function
        write_to_file(dyn_img);

        // 3. Read output and check
        let output = fs::read_to_string("hello.txt").expect("couldn't read file");

        let expected = format!(
            "{}{}\n{}{}\n",
            bright_to_ascii(image::Rgba([0, 0, 0, 255])),
            bright_to_ascii(image::Rgba([255, 255, 255, 255])),
            bright_to_ascii(image::Rgba([127, 127, 127, 255])),
            bright_to_ascii(image::Rgba([200, 200, 200, 255])),
        );

        assert_eq!(output, expected);
    }
}
