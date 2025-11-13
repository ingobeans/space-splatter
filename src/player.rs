use macroquad::prelude::*;

use crate::{
    assets::{Assets, Chunk, World},
    utils::*,
};

pub struct Player {
    pub pos: Vec2,
    pub camera_pos: Vec2,
    pub velocity: Vec2,
    pub animation_time: f32,
    pub walking: bool,
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            camera_pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            animation_time: 0.0,
            walking: false,
        }
    }
    pub fn update(&mut self, delta_time: f32, world: &World) {
        self.animation_time += delta_time;
        self.walking = false;
        let axis = get_input_axis();
        if axis.length() > 0.0 {
            self.walking = true;
            self.velocity += axis.normalize() * delta_time * 3600.0;
        }

        let friction = if axis.length() == 0.0 { 20.0 } else { 10.0 } * delta_time;
        self.velocity = self
            .velocity
            .clamp_length_max(2.0 * 70.0)
            .lerp(Vec2::ZERO, friction);
        let new = update_physicsbody(self.pos, &mut self.velocity, delta_time, &world.collision);
        self.pos = new;
        self.camera_pos = self.pos
    }
    pub fn draw(&self, assets: &Assets) {
        draw_texture(
            assets.player.animations[if self.walking { 1 } else { 0 }]
                .get_at_time((self.animation_time * 1000.0) as u32),
            self.pos.x.floor(),
            self.pos.y.floor(),
            WHITE,
        );
    }
}
fn ceil_g(a: f32) -> f32 {
    if a < 0.0 { a.floor() } else { a.ceil() }
}
fn get_tile(chunks: &[&Chunk], x: i16, y: i16) -> i16 {
    let cx = ((x as f32 / 16.0).floor() * 16.0) as i16;
    let cy = ((y as f32 / 16.0).floor() * 16.0) as i16;
    let Some(chunk) = chunks.iter().find(|f| f.x == cx && f.y == cy) else {
        return 0;
    };
    let local_x = x - chunk.x;
    let local_y = y - chunk.y;
    chunk.tile_at(local_x as _, local_y as _).unwrap_or(0)
}

pub fn update_physicsbody(
    pos: Vec2,
    velocity: &mut Vec2,
    delta_time: f32,
    collision_tiles: &[Chunk],
) -> Vec2 {
    let mut new = pos + *velocity * delta_time;

    let tile_x = pos.x / 16.0;
    let tile_y = pos.y / 16.0;

    let tiles_y = [
        (tile_x.trunc(), ceil_g(new.y / 16.0)),
        (ceil_g(tile_x), ceil_g(new.y / 16.0)),
        (tile_x.trunc(), (new.y / 16.0).trunc()),
        (ceil_g(tile_x), (new.y / 16.0).trunc()),
    ];

    let chunks_pos: [(i16, i16); 4] = std::array::from_fn(|f| {
        let cx = ((tiles_y[f].0 / 16.0).floor() * 16.0) as i16;
        let cy = ((tiles_y[f].1 / 16.0).floor() * 16.0) as i16;
        (cx, cy)
    });

    let chunks: Vec<&Chunk> = collision_tiles
        .iter()
        .filter(|f| chunks_pos.contains(&(f.x, f.y)))
        .collect();

    for (tx, ty) in tiles_y {
        let tile = get_tile(&chunks, tx as i16, ty as i16);
        if tile != 0 {
            let c = if velocity.y < 0.0 {
                tile_y.floor() * 16.0
            } else {
                tile_y.ceil() * 16.0
            };
            new.y = c;
            velocity.y = 0.0;
            break;
        }
    }
    let tiles_x = [
        ((new.x / 16.0).trunc(), ceil_g(new.y / 16.0)),
        (ceil_g(new.x / 16.0), ceil_g(new.y / 16.0)),
        (ceil_g(new.x / 16.0), (new.y / 16.0).trunc()),
        ((new.x / 16.0).trunc(), (new.y / 16.0).trunc()),
    ];

    let chunks_pos: [(i16, i16); 4] = std::array::from_fn(|f| {
        let cx = ((tiles_x[f].0 / 16.0).floor() * 16.0) as i16;
        let cy = ((tiles_x[f].1 / 16.0).floor() * 16.0) as i16;
        (cx, cy)
    });

    let chunks: Vec<&Chunk> = collision_tiles
        .iter()
        .filter(|f| chunks_pos.contains(&(f.x, f.y)))
        .collect();

    for (tx, ty) in tiles_x {
        let tile = get_tile(&chunks, tx as i16, ty as i16);
        if tile != 0 {
            let c = if velocity.x < 0.0 {
                tile_x.floor() * 16.0
            } else {
                tile_x.ceil() * 16.0
            };
            new.x = c;
            velocity.x = 0.0;
            break;
        }
    }
    new
}
