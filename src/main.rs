/// ANON SAY - Meme Generator CLI in Rust
/// Usage:
/// anon-say --image path/to/image.jpg --top-text "Top" --bottom-text "Bottom"

use clap::Parser;
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::fs;

#[derive(Parser)]
#[command(name = "ANON SAY", version, author, about = "A simple meme generator in Rust")]
struct Args {
    #[arg(short, long)]
    image: String,

    #[arg(short = 't', long, default_value = "")]
    top_text: String,

    #[arg(short = 'b', long, default_value = "")]
    bottom_text: String,

    #[arg(short, long, default_value = "output.png")]
    output: String,

    #[arg(short = 's', long, default_value_t = 72.0)]
    font_size: f32,

    #[arg(long, default_value_t = false)]
    auto_font: bool,

    #[arg(long, default_value = "FFFFFF")]
    font_color: String,

    #[arg(long, default_value = "000000")]
    outline_color: String,

    #[arg(long, default_value_t = 3)]
    outline_thickness: i32,

    #[arg(long, default_value = "fonts/NotoSansTC-Regular.ttf")]
    font_path: String,
}

fn parse_hex_color(hex: &str) -> Rgba<u8> {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    Rgba([r, g, b, 255])
}

fn draw_centered_text_with_outline(
    img: &mut RgbaImage,
    text: &str,
    y: i32,
    font: &Font,
    scale: Scale,
    font_color: Rgba<u8>,
    outline_color: Rgba<u8>,
    outline_thickness: i32,
) {
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font.layout(text, scale, rusttype::point(0.0, v_metrics.ascent)).collect();
    let text_width = glyphs
        .iter()
        .rev()
        .find_map(|g| g.pixel_bounding_box().map(|bb| bb.max.x as f32))
        .unwrap_or(0.0);

    let x = ((img.width() as f32 - text_width) / 2.0).max(0.0) as i32;
    let text_x = x;
    let text_y = y;

    let mut offsets = vec![];
    for dx in -outline_thickness..=outline_thickness {
        for dy in -outline_thickness..=outline_thickness {
            if dx != 0 || dy != 0 {
                offsets.push((dx, dy));
            }
        }
    }
    for (dx, dy) in &offsets {
        draw_text_mut(img, outline_color, text_x + dx, text_y + dy, scale, font, text);
    }

    draw_text_mut(img, font_color, text_x, text_y, scale, font, text);
}

fn main() {
    let args = Args::parse();

    let mut img = image::open(&args.image)
        .expect("Failed to open image")
        .to_rgba8();

    let font_data = fs::read(&args.font_path).expect("Font file not found");
    let font = Font::try_from_vec(font_data).expect("Invalid font file");

    let scale = if args.auto_font {
        let dynamic_size = (img.height().min(img.width()) as f32) / 8.0;
        Scale::uniform(dynamic_size)
    } else {
        Scale::uniform(args.font_size)
    };

    let font_color = parse_hex_color(&args.font_color);
    let outline_color = parse_hex_color(&args.outline_color);

    if !args.top_text.is_empty() {
        draw_centered_text_with_outline(
            &mut img,
            &args.top_text,
            10,
            &font,
            scale,
            font_color,
            outline_color,
            args.outline_thickness,
        );
    }

    if !args.bottom_text.is_empty() {
        let h = img.height() as i32;
        let y = h - (scale.y as i32 + 10);
        draw_centered_text_with_outline(
            &mut img,
            &args.bottom_text,
            y,
            &font,
            scale,
            font_color,
            outline_color,
            args.outline_thickness,
        );
    }

    img.save(&args.output).expect("Failed to save output image");
    println!("✅ Meme saved to {}", &args.output);
}
