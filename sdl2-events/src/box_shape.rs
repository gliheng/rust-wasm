use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

pub struct Box {
    shape: Rect,
    color: Color,
    dx: i32,
    dy: i32,
    dragging: bool
}

impl Box {
    fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Box {
        Box{
            shape: Rect::new(x, y, width, height),
            color: color,
            dx: 0,
            dy: 0,
            dragging: false
        }
    }

    pub fn from_center<P>(center: P, width: u32, height: u32, color: Color) -> Box
        where P: Into<Point> {
        Box{shape: Rect::from_center(center, width, height),
            color: color,
            dx: 0,
            dy: 0,
            dragging: false
        }
    }

    pub fn drag(&self) -> bool {
        self.dragging
    }

    pub fn set_drag(&mut self, drag: bool) {
        self.dragging = drag;
    }

    pub fn x(&self) -> i32 {
        self.shape.x()
    }

    pub fn y(&self) -> i32 {
        self.shape.y()
    }

    pub fn set_x(&mut self, x: i32) {
        self.shape.set_x(x)
    }

    pub fn set_y(&mut self, y: i32) {
        self.shape.set_y(y)
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
    pub fn confine(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let x = self.x();
        let y = self.y();
        let w = self.width() as i32;
        let h = self.height() as i32;
        if x < 0 {
            self.set_x(-x);
            self.dx = -self.dx;
        } else if x + w > x1 {
            self.set_x(2 * x1 - x - 2 * w);
            self.dx = -self.dx;
        }
        if y < 0 {
            self.set_y(-y);
            self.dy = -self.dy;
        } else if y + h > y1 {
            self.set_y(2 * y1 - y - 2 * h);
            self.dy = -self.dy;
        }
    }

    pub fn update(&mut self) {
        if self.dragging || self.dx == 0 && self.dy == 0 {
            return;
        }
        let x = self.x() + self.dx;
        let y = self.y() + self.dy;
        self.set_x(x);
        self.set_y(y);
        let friction = 1;
        self.dx = apply_friction(friction, self.dx);
        self.dy = apply_friction(friction, self.dy);
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.dx = x - self.x();
        self.dy = y - self.y();
        self.shape.set_x(x);
        self.shape.set_y(y);
    }

    pub fn point_inside(&self, x: i32, y: i32) -> bool {
        let shape = &self.shape;
        x >= shape.x() && x <= shape.x() + shape.width() as i32
            && y >= shape.y() && y <= shape.y() + shape.height() as i32
    }
}

fn apply_friction(friction: u32, dx: i32) -> i32 {
    let friction = friction as i32;
    if dx.abs() < friction {
        0
    } else {
        dx + (if dx > 0 {-friction} else {friction})
    }
}
