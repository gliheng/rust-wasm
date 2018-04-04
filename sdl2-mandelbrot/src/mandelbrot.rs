use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{ TextureCreator, Texture, RenderTarget };
use sdl2::rect::Rect;
use num::Complex;
use utils;


pub struct Mandelbrot {
    texture: Texture,
}

impl Mandelbrot {
    pub fn new (canvas: &Canvas<Window>) -> Self {

        let creator = canvas.texture_creator();
        let (width, height) = utils::get_window_dimention();
        let mut texture = creator.create_texture_streaming(Some(PixelFormatEnum::RGB24),
                                         width,
                                         height).unwrap();

        texture.with_lock(Rect::new(0, 0, 10, 10), |data: &mut [u8], pitch: usize| {
            println!("len: {}", data.len());
            for (i, d) in data.iter_mut().enumerate() {
                if i % 3 == 0 {
                    *d = 255;
                }
            }
        });
        // surface.with_lock_mut(|data: &mut [u8]| {
        //     for (i, d) in data.iter_mut().enumerate() {
        //         if i % 3 == 0 {
        //             *d = 255;
        //         }
        //     }
        //     println!("data {:?}", data);
        // });
        Mandelbrot {
            texture
        }
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.copy(&self.texture, None, None);
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
