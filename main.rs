use std::io::{stdin, stdout, Write};
use std::time::Duration;
use image::{open, ColorType};

fn main() {
    print!("Enter image file name: ");
    stdout().flush().unwrap();
    let mut filename = String::new();
    stdin().read_line(&mut filename).unwrap();

    let mut image = if let Ok(img) = open(filename.trim()) {
        img
    } else {
        eprintln!("Failed to load image!");
        return;
    };

    image = image.resize(80, 70, image::imageops::FilterType::Nearest);

    match image.color() {
        ColorType::Rgb8 => {
            let data = image.as_rgb8().unwrap().as_raw();
            for y in 0..image.height() {
                for x in 0..image.width() {
                    let index = (y * image.width() + x) as usize * 3;
                    let r = data[index] as f32 / 255.0;
                    let g = data[index + 1] as f32 / 255.0;
                    let b = data[index + 2] as f32 / 255.0;
                    print!("{}", get_char_from_color(r, g, b));
                }
                println!();
            }
        }
        ColorType::Rgba8 => {
            let data = image.as_rgba8().unwrap().as_raw();
            for y in 0..image.height() {
                for x in 0..image.width() {
                    let index = (y * image.width() + x) as usize * 4;
                    let r = data[index] as f32 / 255.0;
                    let g = data[index + 1] as f32 / 255.0;
                    let b = data[index + 2] as f32 / 255.0;
                    let a = data[index + 3] as f32 / 255.0;
                    print!("{}", get_char_from_color_alpha(r, g, b, a));
                }
                println!();
            }
        }
        _ => eprintln!("Unsupported image format!"),
    }

    std::thread::sleep(Duration::from_secs(10000));
}


fn get_char_from_color(r: f32, g: f32, b: f32) -> char {
    let gray = (r * 0.299 + g * 0.587 + b * 0.114) * 255.0;
    match gray as u32 {
        0..=29 => '█',
        30..=59 => '▓',
        60..=119 => '▒',
        _ => '░'
    }
}

fn get_char_from_color_alpha(r: f32, g: f32, b: f32, a: f32) -> char {
    if a < 0.5 {
        ' '
    } else {
        get_char_from_color(r, g, b)
    }
}
