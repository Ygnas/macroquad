use macroquad::prelude::{BLUE, draw_rectangle};

use crate::map::Renderable;

pub struct Unit {
    x: f32,
    y: f32,
}

impl Unit {
    pub fn new(x: f32, y: f32) -> Unit {
        Unit { x, y }
    }
}

impl Renderable for Unit {
    fn render(&self) {
        draw_rectangle(self.x*32.0, self.y*32.0, 32.0, 32.0, BLUE)
    }

    fn render_texture(&self, _texture: macroquad::prelude::Texture2D) {
        todo!()
    }
}