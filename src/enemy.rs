use std::{collections::VecDeque, iter::Map};

use crate::{
    assets::{Assets, World},
    player::Player,
    utils::*,
};
use macroquad::prelude::*;

pub struct EnemyType {
    pub draw_fn: &'static dyn Fn(&Assets, &Enemy),
    pub speed: f32,
}

pub const GREENO: EnemyType = EnemyType {
    draw_fn: &|assets, enemy| {
        draw_texture_ex(
            assets.enemies.animations[0].get_at_time((enemy.animation_time * 1000.0) as u32),
            enemy.pos.x.floor() - 16.0,
            enemy.pos.y.floor() - 16.0,
            WHITE,
            DrawTextureParams {
                flip_x: enemy.moving_left,
                ..Default::default()
            },
        );
    },
    speed: 50.0,
};

pub struct Enemy {
    pub ty: &'static EnemyType,
    pub pos: Vec2,
    pub health: f32,
    pub animation_time: f32,
    pub moving_left: bool,
    pub path: Option<VecDeque<(i16, i16)>>,
    pub time_til_pathfind: f32,
}
impl Enemy {
    pub fn update(&mut self, delta_time: f32, player: &mut Player, world: &World) {
        let delta = player.pos - self.pos;
        if delta.length() > 0.0 {
            self.animation_time += delta_time;
            self.time_til_pathfind -= delta_time;
            self.moving_left = delta.x < 0.0;

            if self.path.is_none() || self.time_til_pathfind <= 0.0 {
                self.time_til_pathfind = 2.0;
                self.path = world.pathfind(self.pos, player.pos).map(|f| f.0.into());
            }
            if let Some(path) = &mut self.path
                && let Some((x, y)) = path.get(1)
            {
                let next = vec2(*x as f32 * 16.0, *y as f32 * 16.0);
                if next.distance(self.pos) < 4.0 {
                    path.pop_front();
                }

                self.pos += (next - self.pos).normalize() * delta_time * self.ty.speed;
            }
        }
    }
    pub fn draw(&self, assets: &Assets) {
        (self.ty.draw_fn)(assets, &self);
    }
}
