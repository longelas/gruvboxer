use image::{RgbImage, Pixel};
use palette::{Lab, Srgb, FromColor, white_point::D65};
use palette::color_difference::DeltaE;
use std::env;

// Enhanced Gruvbox palette
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

fn enhanced_harmonization(original: Lab<D65>, strength: f32) -> Lab<D65> {
    let target = GRUVBOX_LAB.iter()
        .min_by_key(|&&c| (original.delta_e(c) * 1000.0) as u32)
        .unwrap();

    Lab::new(
        original.l * 0.9 + target.l * 0.1,
        original.a * (1.0 - strength) + target.a * strength * 1.5,
        original.b * (1.0 - strength) + target.b * strength * 1.5,
    )
}

fn apply_enhanced_harmonization(img: &mut RgbImage, strength: f32) {
    let strength = strength.clamp(0.0, 2.0);
    
    for pixel in img.pixels_mut() {
        let rgb = pixel.to_rgb();
        let srgb = Srgb::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0
        ).into_format();

        let original_lab: Lab<D65> = Lab::from_color(srgb);
        let harmonized = enhanced_harmonization(original_lab, strength);
        let result_rgb = Srgb::from_color(harmonized).into_format::<f32>();

        pixel[0] = (result_rgb.red * 255.0).clamp(0.0, 255.0) as u8;
        pixel[1] = (result_rgb.green * 255.0).clamp(0.0, 255.0) as u8;
        pixel[2] = (result_rgb.blue * 255.0).clamp(0.0, 255.0) as u8;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input> [output] [strength=1.0]", args[0]);
        return Ok(());
    }

    let strength = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1.0);
    let mut img = image::open(&args[1])?.into_rgb8();
    
    apply_enhanced_harmonization(&mut img, strength);
    
    let output_path = args.get(2).map(|s| s.as_str()).unwrap_or("output.png");
    img.save(output_path)?;
    
    println!("Saved enhanced image to {}", output_path);
    Ok(())
}