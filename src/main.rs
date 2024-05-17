use num::Complex;
use std::env;

mod mandel_brot;
use mandel_brot::parse_complex;
use mandel_brot::parse_pair;
use mandel_brot::render_and_write_image;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprint!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprint!("Example: {} mandel.png 1000x750 -1.20,0.35 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds: (usize, usize)= parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right: Complex<f64> = parse_complex(&args[4]).expect("error parsing lower right corner point");

    render_and_write_image(bounds, upper_left, lower_right, &args[1]);
    
    
}


    