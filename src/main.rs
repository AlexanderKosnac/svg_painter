use std::fs;
use std::env;

use svg_painter::approximation::{FileType, ImageApproximation};

use svg_painter::run;

static BUILD: &str = "build";

fn main() {
    let args: Vec<String> = env::args().collect();

    let raster_image_path = &args[1];

    fs::create_dir_all(String::from(BUILD)).expect("Unable to create build directory");
    fs::copy(raster_image_path, format!("{BUILD}/trgt.png")).expect("Could not copy target file");

    run(&svg_painter::util::read_image(raster_image_path), |img_approx: &ImageApproximation| {
        img_approx.write_to_file(&FileType::SVG, &format!("{BUILD}/expr.svg"));
        img_approx.write_to_file(&FileType::PNG, &format!("{BUILD}/expr.png"));
    });
}
