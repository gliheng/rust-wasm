use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

pub struct Box {
    shape: Rect,
    color: Color,
    px: f32,
    py: f32,
    dx: f32,
    dy: f32,
    dragging: bool
}

impl Box {
    fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Box {
        Box{
            shape: Rect::new(x, y, width, height),
            color: color,
            px: x as f32,
            py: y as f32,
            dx: 0f32,
            dy: 0f32,
            dragging: false
        }
    }

    pub fn from_center<P>(center: P, width: u32, height: u32, color: Color) -> Box
        where P: Into<Point> {
        let shape = Rect::from_center(center, width, height);
        Box{shape: shape,
            color: color,
            px: shape.x() as f32,
            py: shape.y() as f32,
            dx: 0f32,
            dy: 0f32,
            dragging: false
        }
    }

    pub fn drag(&self) -> bool {
        self.dragging
    }

    pub fn set_drag(&mut self, drag: bool) {
        self.dragging = drag;
    }

    pub fn x(&self) -> f32 {
        self.px as f32
    }

    pub fn y(&self) -> f32 {
        self.py as f32
    }

    pub fn set_x(&mut self, x: f32) {
        self.shape.set_x(x as i32);
        self.px = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.shape.set_y(y as i32);
        self.py = y;
    }

    pub fn width(&self) -> u32 {
        self.shape.width()
    }

    pub fn height(&self) -> u32 {
        self.shape.height()
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(self.shape);
    }

    /// confine motion within a boundry
    /// mirror motion on border
    pub fn confine(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        let x = self.x();
        let y = self.y();
        let w = self.width() as f32;
        let h = self.height() as f32;
        if x < 0f32 {
            self.set_x(-x);
            self.dx = -self.dx;
        } else if x + w > x1 {
            self.set_x(2f32 * x1 - x - 2f32 * w);
            self.dx = -self.dx;
        }
        if y < 0f32 {
            self.set_y(-y);
            self.dy = -self.dy;
        } else if y + h > y1 {
            self.set_y(2f32 * y1 - y - 2f32 * h);
            self.dy = -self.dy;
        }
    }

    pub fn update(&mut self) {
        if self.dragging || self.dx.abs() < 0.00001 && self.dy.abs() < 0.00001 {
            return;
        }

        let x = self.x() + self.dx;
        let y = self.y() + self.dy;
        self.set_x(x);
        self.set_y(y);
        let friction = 1.0;
        self.dx = apply_friction(friction, self.dx);
        self.dy = apply_friction(friction, self.dy);
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.dx = x - self.x();
        self.dy = y - self.y();
        self.set_x(x);
        self.set_y(y);
    }

    pub fn point_inside(&self, x: f32, y: f32) -> bool {
        x >= self.x() && x <= self.x() + self.width() as f32
            && y >= self.y() && y <= self.y() + self.height() as f32
    }
}

fn apply_friction(friction: f32, dx: f32) -> f32 {
    if dx.abs() < friction {
        0f32
    } else {
        dx + (if dx > 0f32 {-friction} else {friction})
    }
}
