use macroquad::prelude::*;

use crate::{assets::Assets, utils::*};

pub struct Player {
    pub pos: Vec2,
    pub camera_pos: Vec2,
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            camera_pos: Vec2::ZERO,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let axis = get_input_axis();
        if axis.length() > 0.0 {
            self.pos += axis.normalize() * delta_time * 80.0;
        }
        self.camera_pos = self.pos
    }
    pub fn draw(&self, assets: &Assets) {
        assets
            .tileset
            .draw_sprite(self.pos.x.floor(), self.pos.y.floor(), 0.0, 1.0, None);
    }
}
