use structopt::*;
use num_complex::Complex;
use rayon::prelude::*;

#[derive(Debug, StructOpt)]
struct Opt {
    sx : f64,
    ex : f64, 
    sy : f64,
    ey : f64,
    #[structopt(long="max-iter", default_value="80")]
    max_iter : usize,
    w : u32,
    h : u32,
}

fn mandelbrot(c : Complex<f64>, max_iter : usize) -> usize {
    let mut z = Complex::from(0f64);
    let mut n = 0;
    while z.norm() <= 2f64 && n < max_iter {
        z = z*z + c;
        n += 1;
    }
    n
}

fn main() {    
    let opt = Opt::from_args();

    let s = Complex::new(opt.sx, opt.sy);
    let e = Complex::new(opt.ex, opt.ey);
    let size = Complex::new(opt.w as f64, opt.h as f64);
    let delta = (e-s);
    let scal = Complex::new(delta.re / size.re, delta.im / size.im);

    let rt = (0..opt.h).into_par_iter().map(|y| 
    //let rt = (0..opt.h).map(|y| 
        (0..opt.w).into_par_iter().map(|x| {
            let it = mandelbrot(s + Complex::new(scal.re*x as f64, scal.im*y as f64), opt.max_iter);

            (opt.max_iter as f64 * 255f64 / it as f64) as u8
        }).collect::<Vec<u8>>()
    ).collect::<Vec<_>>();
    

    serde_json::to_writer(std::io::stdout(), &rt);
}
