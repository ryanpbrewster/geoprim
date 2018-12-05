use image;
use image::GenericImageView;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let options: CliOptions = CliOptions::from_args();
    println!("{:?}", options);

    let img = image::open(options.source_image)
        .expect("open source_image")
        .resize(400, 300, image::FilterType::Gaussian)
        .grayscale();
    println!("{:?}", img.dimensions());
}

#[derive(StructOpt, Debug)]
struct CliOptions {
    #[structopt(name = "source_image", parse(from_os_str))]
    source_image: PathBuf,
}
