use num::Complex;
use sdl2::render::Canvas;
use sdl2::video::{ Window, WindowContext };
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{ TextureCreator, Texture, RenderTarget };
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use utils;


pub struct Mandelbrot<'a> {
    creator: TextureCreator<WindowContext>,
    surface: Option<Surface<'a>>,
    texture: Option<Texture>,
    r1: Complex<f64>,
    r2: Complex<f64>,
}

impl<'a> Mandelbrot<'a> {
    pub fn new (canvas: &Canvas<Window>) -> Self {

        let creator = canvas.texture_creator();

        let r1 = Complex {re: -2.0, im: -1.0};
        let r2 = Complex {re: 1.0, im: 1.0};
        let mut inst = Mandelbrot {
            creator,
            surface: None,
            texture: None,
            r1,
            r2,
        };

        inst.update(r1, r2);

        inst
    }

    pub fn update(&mut self, r1: Complex<f64>, r2: Complex<f64>) {
        let (width, height) = utils::get_window_dimention();

        let mut surface = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();
        surface.with_lock_mut(|data: &mut [u8]| {

            for i in 0..(data.len() / 3) {
                let x = i % width as usize;
                let y = i / width as usize;
                let point = pixel_to_point(x, y, (width as usize, height as usize), self.r1, self.r2);
                let v = match escape_time(point, 100) {
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

        self.surface = Some(surface);
        self.texture = Some(texture);

    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        if let Some(ref tex) = self.texture {
            canvas.copy(tex, None, None);
        }
    }
}


fn pixel_to_point(x: usize,
                  y: usize,
                  bounds: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>) -> Complex<f64> {

    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + x as f64 * width / bounds.0 as f64,
        im: upper_left.im - y as f64 * height / bounds.1 as f64,
    }
}

pub fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}
