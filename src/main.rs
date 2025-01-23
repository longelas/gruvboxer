use image::{RgbImage, Pixel, imageops};
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
    Lab::new(29.77, 0.16, 0.20),     // background
    Lab::new(86.97, -0.86, 9.92),    // foreground
    Lab::new(44.36, 55.40, 37.13),   // red
    Lab::new(56.83, -21.99, 56.27),  // green
    Lab::new(65.17, 10.15, 57.42),   // yellow
    Lab::new(49.59, -9.26, -24.91),  // blue
    Lab::new(51.70, 34.04, -14.60),  // purple
    Lab::new(58.69, -28.30, 15.25),  // aqua
    Lab::new(53.33, 39.77, 52.78),   // orange
];

fn apply_style(img: &mut RgbImage, style: Style, strength: f32) {
    match style {
        Style::Gruvbox => {
            let gray = imageops::colorops::grayscale(img);
            let filtered = imageproc::filter::bilateral_filter(&gray, 5, 25.0, 2.0);
            enhanced_harmonization(img, strength, Some(&filtered));
        }
        Style::Retro => {
            let gray = imageops::colorops::grayscale(img);
            let filtered = imageproc::filter::bilateral_filter(&gray, 3, 15.0, 1.5);
            enhanced_harmonization(img, strength, Some(&filtered));
            add_vhs_effect(img);
            add_film_grain(img, 12);
        }
        Style::Synthwave => {
            enhanced_harmonization(img, strength * 0.8, None);
            apply_gradient_overlay(img);
            boost_saturation(img, 1.5);
        }
        Style::Mosaic(size) => {
            enhanced_harmonization(img, strength, None);
            pixelate(img, size);
        }
        Style::Watercolor => {
            enhanced_harmonization(img, strength, None);
            apply_watercolor_effect(img);
        }
    }
}

fn enhanced_harmonization(
    img: &mut RgbImage,
    strength: f32,
    edge_mask: Option<&image::GrayImage>,
) {
    let strength = strength.clamp(0.0, 1.0);
    let contrast_boost = 1.08;
    
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let rgb = pixel.to_rgb();
        let srgb = Srgb::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0
        ).into_format();

        let original_lab: Lab<D65> = Lab::from_color(srgb);
        let mut harmonized = harmonize_color(original_lab, strength);
        
        // Edge-aware strength adjustment
        if let Some(mask) = edge_mask {
            let edge_strength = mask.get_pixel(x, y)[0] as f32 / 255.0;
            harmonized.l = original_lab.l * (1.0 - edge_strength) + harmonized.l * edge_strength;
        }

        let mut result_rgb = Srgb::from_color(harmonized).into_format::<f32>();
        
        // Contrast compensation
        result_rgb.red = (result_rgb.red * contrast_boost).clamp(0.0, 1.0);
        result_rgb.green = (result_rgb.green * contrast_boost).clamp(0.0, 1.0);
        result_rgb.blue = (result_rgb.blue * contrast_boost).clamp(0.0, 1.0);

        pixel[0] = (result_rgb.red * 255.0) as u8;
        pixel[1] = (result_rgb.green * 255.0) as u8;
        pixel[2] = (result_rgb.blue * 255.0) as u8;
    }
}

fn harmonize_color(original: Lab<D65>, strength: f32) -> Lab<D65> {
    let target = GRUVBOX_LAB.iter()
        .min_by_key(|&&c| (original.delta_e(c) * 1000.0) as u32)
        .unwrap();

    // Sigmoid-based blending for smooth transitions
    fn blend_channel(orig: f32, tgt: f32, strength: f32) -> f32 {
        let mix = strength * (1.0 - (-4.0 * (orig - tgt).abs()).exp()).recip();
        orig * (1.0 - mix) + tgt * mix
    }

    Lab::new(
        original.l * 0.98 + target.l * 0.02,
        blend_channel(original.a, target.a, strength),
        blend_channel(original.b, target.b, strength),
    )
}

fn add_vhs_effect(img: &mut RgbImage) {
    let (width, height) = img.dimensions();
    let mut shifted_r = img.clone();
    let mut shifted_b = img.clone();

    shifted_r = imageops::crop(&mut shifted_r, 2, 0, width-2, height).to_image();
    shifted_b = imageops::crop(&mut shifted_b, 0, 1, width, height-1).to_image();

    imageops::overlay(img, &shifted_r, 0, 0);
    imageops::overlay(img, &shifted_b, 0, 0);

    for (_, y, pixel) in img.enumerate_pixels_mut() {
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
    let small = imageops::resize(
        &*img,
        w / block_size,
        h / block_size,
        imageops::FilterType::Nearest,
    );
    *img = imageops::resize(&small, w, h, imageops::FilterType::Nearest);
}

fn apply_watercolor_effect(img: &mut RgbImage) {
    let blurred = imageops::blur(img, 2.0);
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