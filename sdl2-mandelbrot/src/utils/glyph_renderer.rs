///! GlyphRenderer render a string character by character
///! using cached Texture for each character.

use sdl2::ttf::{ Font };
use std::collections::HashMap;
use sdl2::render::Texture;
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;

pub struct GlyphRenderer<'ttf> {
    tc: TextureCreator<WindowContext>,
    font: Font<'ttf, 'static>,
    color: Color,
    glyphs: HashMap<char, SizedTexture>,
}

impl <'ttf> GlyphRenderer<'ttf> {
    pub fn new(tc: TextureCreator<WindowContext>, font: Font<'ttf, 'static>, color: Color) -> Self {
        Self {
            tc,
            font,
            color,
            glyphs: HashMap::new(),
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>, s: &str, mut x: i32, mut y: i32) {
        for c in s.chars() {
            let m = self.font.find_glyph_metrics(c);
            if m.is_none() { continue; } // font does not exist in font

            if !self.glyphs.contains_key(&c) {
                let surf = self.font.render_char(c).blended(self.color).unwrap();
                self.glyphs.insert(c, SizedTexture(surf.width(),
                                                   surf.height(),
                                                   self.tc.create_texture_from_surface(surf).unwrap()));
            }

            let &SizedTexture(w, h, ref tex) = self.glyphs.get(&c).unwrap();
            let rect = Rect::new(x, y, w, h);
            let _ = canvas.copy(tex,
                        None,
                        rect);

            x += m.unwrap().advance;
        }
    }
}

pub struct SizedTexture(pub u32, pub u32, pub Texture);
unsafe impl Send for SizedTexture {}
