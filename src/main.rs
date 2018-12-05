use image;
use image::{GenericImageView, GrayImage, ImageBuffer};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let options: CliOptions = CliOptions::from_args();
    println!("{:?}", options);

    let target = image::open(options.source_image)
        .expect("open source_image")
        .resize(400, 300, image::FilterType::Gaussian)
        .to_luma();

    let (width, height) = target.dimensions();
    let work: GrayImage = ImageBuffer::new(width, height);

    let (cx, cy) = (width / 2, height / 2);

    let mut best_shape = Shape::Ellipse {
        cx,
        cy,
        rx: 1,
        ry: 1,
    };
    let mut best_score = score_shape(&target, &work, &best_shape);
    for rx in 1..cx {
        for ry in 1..cy {
            let shape = Shape::Ellipse { cx, cy, rx, ry };
            let score = score_shape(&target, &work, &shape);
            println!("{:?} -> {}", shape, score);
            if score < best_score {
                best_score = score;
                best_shape = shape;
            }
        }
    }
    println!("{:?} -> {}", best_shape, best_score);
}

fn score_shape(target: &GrayImage, source: &GrayImage, shape: &Shape) -> u64 {
    assert_eq!(source.dimensions(), target.dimensions());
    let (width, height) = source.dimensions();
    let mut total: u64 = 0;
    for x in 0..width {
        for y in 0..height {
            let s = if shape.contains(x, y) {
                255
            } else {
                source.get_pixel(x, y)[0] as u64
            };
            let t = target.get_pixel(x, y)[0] as u64;
            total += s * s + t * t - 2 * t * s;
        }
    }
    total
}

#[derive(Debug)]
enum Shape {
    Ellipse { cx: u32, cy: u32, rx: u32, ry: u32 },
}

impl Shape {
    fn contains(&self, x: u32, y: u32) -> bool {
        match *self {
            Shape::Ellipse { cx, cy, rx, ry } => {
                let dx2 = x * x + cx * cx - 2 * x * cx;
                let dy2 = y * y + cy * cy - 2 * y * cy;
                dx2 * ry * ry + dy2 * rx * rx <= rx * rx * ry * ry
            }
        }
    }
}

#[derive(StructOpt, Debug)]
struct CliOptions {
    #[structopt(name = "source_image", parse(from_os_str))]
    source_image: PathBuf,
}
