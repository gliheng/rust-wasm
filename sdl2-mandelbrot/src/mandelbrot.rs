use num::Complex;
use std::time::{ Duration, Instant };
use sdl2::render::Canvas;
use sdl2::video::{ Window, WindowContext };
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{ TextureCreator, Texture, RenderTarget };
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use utils;

const R1: Complex<f32> = Complex {re: -2.0, im: -1.0};
const R2: Complex<f32> = Complex {re: 1.0, im: 1.0};
const ITERATIONS: u32 = 150;

pub struct Mandelbrot {
    creator: TextureCreator<WindowContext>,
    texture: Option<Texture>,
    render_time: Duration,
    r1: Complex<f32>,
    r2: Complex<f32>,
}

impl Mandelbrot {
    pub fn new (canvas: &Canvas<Window>) -> Self {

        let creator = canvas.texture_creator();

        let r1 = Complex {re: -2.0, im: -1.0};
        let r2 = Complex {re: 1.0, im: 1.0};
        let mut inst = Mandelbrot {
            creator,
            texture: None,
            render_time: Duration::from_secs(0),
            r1,
            r2,
        };

        inst.update();

        inst
    }

    pub fn update(&mut self) {
        let (width, height) = utils::get_window_dimention();

        let t0 = Instant::now();
        let mut surface = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();
        surface.with_lock_mut(|data: &mut [u8]| {
            for i in 0..(data.len() / 3) {
                let x = i % width as usize;
                let y = i / width as usize;
                let point = pixel_to_point(x, y, (width as usize, height as usize), &self.r1, &self.r2);
                let v = match escape_time(point, ITERATIONS) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };

                data[i * 3] = v;
                data[i * 3 + 1] = v;
                data[i * 3 + 2] = v;
            }
        });

        let texture = self.creator.create_texture_from_surface(&surface)
            .unwrap();

        self.texture = Some(texture);

        self.render_time = t0.elapsed();
        println!("rust mandelbrot render time: {:.3} ms", format_duration(self.render_time) * 1000.);
    }

    pub fn update_rect(&mut self, rect: &Rect) {
        let p1 = rect.top_left();
        let p2 = rect.bottom_right();
        let (width, height) = utils::get_window_dimention();
        let width = width as usize;
        let height = height as usize;
        let r1 = pixel_to_point(p1.x() as usize, p1.y() as usize,
                                (width, height),
                                &self.r1, &self.r2);
        let r2 = pixel_to_point(p2.x() as usize, p2.y() as usize,
                                (width, height) as (usize, usize),
                                &self.r1, &self.r2);

        self.r1 = r1;
        self.r2 = r2;
        self.update();
    }

    pub fn reset(&mut self) {
        if self.r1 == R1 && self.r2 == R2 {
            // no need to reset
            return;
        }
        self.r1 = R1;
        self.r2 = R2;
        self.update();
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        if let Some(ref tex) = self.texture {
            let _ = canvas.copy(tex, None, None);
        }
    }
}


fn format_duration(dur: Duration) -> f32 {
    dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.
}

fn pixel_to_point(x: usize,
                  y: usize,
                  bounds: (usize, usize),
                  top_left: &Complex<f32>,
                  bottom_right: &Complex<f32>) -> Complex<f32> {

    let (width, height) = (bottom_right.re - top_left.re,
                           top_left.im - bottom_right.im);
    Complex {
        re: top_left.re + x as f32 * width / bounds.0 as f32,
        im: top_left.im - y as f32 * height / bounds.1 as f32,
    }
}

pub fn escape_time(c: Complex<f32>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::Complex;
    use super::escape_time;
    #[test]
    fn it_works() {
        println!("{:?}", escape_time(Complex{re: -1.0, im: 1.0}, 50));
        println!("{:?}", escape_time(Complex{re: -1.0, im: 0.7}, 50));
    }
}
