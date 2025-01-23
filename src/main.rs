use image::{RgbImage, Pixel};
use palette::{Lab, Srgb, FromColor, white_point::D65, Hsl};
use palette::color_difference::DeltaE;
use rand::Rng;
use std::env;

#[derive(Clone, Copy)]
enum Style {
    Gruvbox,
    Retro,
    Synthwave,
    Mosaic(u32),
    Watercolor,
}

const GRUVBOX_LAB: [Lab<D65>; 9] = [
    Lab::new(29.77, 2.16, 1.20),
    Lab::new(86.97, -3.86, 12.92),
    Lab::new(44.36, 65.40, 47.13),
    Lab::new(56.83, -31.99, 66.27),
    Lab::new(65.17, 20.15, 67.42),
    Lab::new(49.59, -19.26, -34.91),
    Lab::new(51.70, 44.04, -24.60),
    Lab::new(58.69, -38.30, 25.25),
    Lab::new(53.33, 49.77, 62.78),
];

fn apply_style(img: &mut RgbImage, style: Style, strength: f32) {
    match style {
        Style::Gruvbox => enhanced_harmonization(img, strength),
        Style::Retro => {
            enhanced_harmonization(img, strength);
            add_vhs_effect(img);
            add_film_grain(img, 15);
        }
        Style::Synthwave => {
            enhanced_harmonization(img, strength * 0.8);
            apply_gradient_overlay(img);
            boost_saturation(img, 1.5);
        }
        Style::Mosaic(size) => {
            enhanced_harmonization(img, strength);
            pixelate(img, size);
        }
        Style::Watercolor => {
            enhanced_harmonization(img, strength);
            apply_watercolor_effect(img);
        }
    }
}

fn enhanced_harmonization(img: &mut RgbImage, strength: f32) {
    let strength = strength.clamp(0.0, 2.0);
    
    for pixel in img.pixels_mut() {
        let rgb = pixel.to_rgb();
        let srgb = Srgb::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0
        ).into_format();

        let original_lab: Lab<D65> = Lab::from_color(srgb);
        let harmonized = harmonize_color(original_lab, strength);
        let result_rgb = Srgb::from_color(harmonized).into_format::<f32>();

        pixel[0] = (result_rgb.red * 255.0).clamp(0.0, 255.0) as u8;
        pixel[1] = (result_rgb.green * 255.0).clamp(0.0, 255.0) as u8;
        pixel[2] = (result_rgb.blue * 255.0).clamp(0.0, 255.0) as u8;
    }
}

fn harmonize_color(original: Lab<D65>, strength: f32) -> Lab<D65> {
    let target = GRUVBOX_LAB.iter()
        .min_by_key(|&&c| (original.delta_e(c) * 1000.0) as u32)
        .unwrap();

    Lab::new(
        original.l * 0.9 + target.l * 0.1,
        original.a * (1.0 - strength) + target.a * strength * 1.5,
        original.b * (1.0 - strength) + target.b * strength * 1.5,
    )
}

fn add_vhs_effect(img: &mut RgbImage) {
    let (width, height) = img.dimensions();
    let mut shifted_r = img.clone();
    let mut shifted_b = img.clone();

    shifted_r = image::imageops::crop(&mut shifted_r, 2, 0, width-2, height).to_image();
    shifted_b = image::imageops::crop(&mut shifted_b, 0, 1, width, height-1).to_image();

    image::imageops::overlay(img, &shifted_r, 0, 0);
    image::imageops::overlay(img, &shifted_b, 0, 0);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if y % 2 == 0 {
            pixel[0] = pixel[0].saturating_sub(20);
            pixel[1] = pixel[1].saturating_sub(20);
            pixel[2] = pixel[2].saturating_sub(20);
        }
    }
}

fn add_film_grain(img: &mut RgbImage, intensity: i16) {
    let mut rng = rand::thread_rng();
    for pixel in img.pixels_mut() {
        let noise = rng.gen_range(-intensity..intensity);
        pixel[0] = (pixel[0] as i16 + noise).clamp(0, 255) as u8;
        pixel[1] = (pixel[1] as i16 + noise).clamp(0, 255) as u8;
        pixel[2] = (pixel[2] as i16 + noise).clamp(0, 255) as u8;
    }
}

fn apply_gradient_overlay(img: &mut RgbImage) {
    let height = img.height() as f32;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let position = y as f32 / height;
        let r = (position * 255.0) as u8;
        let b = ((1.0 - position) * 255.0) as u8;
        pixel[0] = pixel[0].saturating_add(r / 2);
        pixel[2] = pixel[2].saturating_add(b / 2);
    }
}

fn boost_saturation(img: &mut RgbImage, factor: f32) {
    for pixel in img.pixels_mut() {
        let rgb = pixel.to_rgb();
        let mut hsl = Hsl::from_color(Srgb::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0
        ));
        
        hsl.saturation *= factor;
        let srgb = Srgb::from_color(hsl);
        
        pixel[0] = (srgb.red * 255.0).clamp(0.0, 255.0) as u8;
        pixel[1] = (srgb.green * 255.0).clamp(0.0, 255.0) as u8;
        pixel[2] = (srgb.blue * 255.0).clamp(0.0, 255.0) as u8;
    }
}

fn pixelate(img: &mut RgbImage, block_size: u32) {
    let (w, h) = img.dimensions();
    let small = image::imageops::resize(
        &*img,
        w / block_size,
        h / block_size,
        image::imageops::FilterType::Nearest,
    );
    *img = image::imageops::resize(&small, w, h, image::imageops::FilterType::Nearest);
}

fn apply_watercolor_effect(img: &mut RgbImage) {
    let blurred = image::imageops::blur(img, 2.0);
    let mut rng = rand::thread_rng();
    
    for y in 0..img.height() {
        for x in 0..img.width() {
            let noise = rng.gen_range(-10..10);
            let pixel = img.get_pixel_mut(x, y);
            let blurred_pixel = blurred.get_pixel(x, y);
            
            pixel[0] = (blurred_pixel[0] as i16 + noise).clamp(0, 255) as u8;
            pixel[1] = (blurred_pixel[1] as i16 + noise).clamp(0, 255) as u8;
            pixel[2] = (blurred_pixel[2] as i16 + noise).clamp(0, 255) as u8;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input> [output] [strength=1.0] [style]", args[0]);
        eprintln!("Available styles:");
        eprintln!("  - gruvbox (default)");
        eprintln!("  - retro");
        eprintln!("  - synthwave");
        eprintln!("  - mosaic");
        eprintln!("  - watercolor");
        return Ok(());
    }

    let strength = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1.0);
    let style = match args.get(4).map(|s| s.as_str()) {
        Some("retro") => Style::Retro,
        Some("synthwave") => Style::Synthwave,
        Some("mosaic") => Style::Mosaic(16),
        Some("watercolor") => Style::Watercolor,
        _ => Style::Gruvbox,
    };

    let mut img = image::open(&args[1])?.into_rgb8();
    apply_style(&mut img, style, strength);
    
    let output_path = args.get(2).map(|s| s.as_str()).unwrap_or("output.png");
    img.save(output_path)?;
    
    println!("Styled image saved to {}", output_path);
    Ok(())
}