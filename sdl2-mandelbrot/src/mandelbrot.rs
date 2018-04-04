use sdl2::render::Canvas;
use sdl2::video::Window;
use num::Complex;

pub struct Mandelbrot {
    
}

impl Mandelbrot {
    pub fn new() -> Self {
        Mandelbrot {
            
        }
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        
    }
}


pub fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0};

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}
