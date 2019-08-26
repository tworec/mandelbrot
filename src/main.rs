use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use num_complex::Complex;
use structopt::*;

#[derive(Debug, StructOpt, Clone)]
struct Opt {
    sx: f64,
    ex: f64,
    sy: f64,
    ey: f64,
    #[structopt(long = "max-iter", default_value = "80")]
    max_iter: usize,
    width: u32,
    height: u32,
}

fn mandelbrot(c: Complex<f64>, max_iter: usize) -> usize {
    let mut z = Complex::from(0f64);
    let mut n = 0;
    while z.norm() <= 2f64 && n < max_iter {
        z = z * z + c;
        n += 1;
    }
    n
}

struct Rect {
    startx: u32,
    starty: u32,
    endx: u32,
    endy: u32
}


struct ExecuteParams {
    s: Complex<f64>,
    scale: Complex<f64>,
    max_iter: usize,
    area: Rect
}

fn exec(params: &ExecuteParams) -> Vec<u8> {
    let data = (params.area.starty..params.area.endy)
        .into_iter()
        .flat_map(|y| {
            let im = params.scale.im * y as f64;
            (params.area.startx..params.area.endx).into_iter().map(move |x| {
                let step = Complex::new(params.scale.re * x as f64, im);
                let it = mandelbrot(params.s + step, params.max_iter);
                //            println!("{}x{}: it = {}", y, x, it);
                (params.max_iter as f64 * 255f64 / it as f64) as u8
            })
        })
        .collect::<Vec<u8>>();

    return data
}


fn main() {
    let opt = Opt::from_args();

    let s = Complex::new(opt.sx, opt.sy);
    let e = Complex::new(opt.ex, opt.ey);
    let size = Complex::new(opt.width as f64, opt.height as f64);
    let delta = e - s;
    let scal = Complex::new(delta.re / size.re, delta.im / size.im);

    let area = Rect{ startx: 0, starty: 0, endx: opt.width, endy: opt.height};
    let params = ExecuteParams{ s, scale: scal, max_iter: opt.max_iter, area };

    let data = exec(&params);

    let path = Path::new("out.png");
    let display = path.display();

    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create file {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut encoder = png::Encoder::new(BufWriter::new(file), opt.width, opt.height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}
