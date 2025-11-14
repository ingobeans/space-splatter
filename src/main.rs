use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{assets::*, enemy::*, player::*, utils::*};

mod assets;
mod enemy;
mod player;
mod ui;
mod utils;

struct Game<'a> {
    assets: &'a Assets,
    world: World,
    player: Player,
    pixel_camera: Camera2D,
    world_camera_bg: Camera2D,
    world_camera_fg: Camera2D,
    stars: StarsBackground,
    enemies: Vec<Enemy>,
    locker_pos: Vec2,
}
impl<'a> Game<'a> {
    fn new(assets: &'a Assets) -> Self {
        let world = World::default();

        let world_width = ((world.x_max - world.x_min) * 16) as f32 + 16.0 * 16.0;
        let world_height = ((world.y_max - world.y_min) * 16) as f32 + 16.0 * 16.0;

        // render world
        let mut world_camera_bg = create_camera(world_width, world_height);
        world_camera_bg.target = vec2(
            (world.x_min + world.x_max + 16) as f32 / 2.0 * 16.0,
            (world.y_min + world.y_max + 16) as f32 / 2.0 * 16.0,
        );
        set_camera(&world_camera_bg);
        clear_background(BLACK.with_alpha(0.0));

        for chunk in &world.background {
            chunk.draw(assets);
        }
        for chunk in &world.collision {
            chunk.draw(assets);
        }
        for chunk in &world.background_details {
            chunk.draw(assets);
        }
        let mut world_camera_fg = create_camera(world_width, world_height);
        world_camera_fg.target = vec2(
            (world.x_min + world.x_max + 16) as f32 / 2.0 * 16.0,
            (world.y_min + world.y_max + 16) as f32 / 2.0 * 16.0,
        );
        set_camera(&world_camera_fg);
        clear_background(BLACK.with_alpha(0.0));
        for chunk in &world.details {
            chunk.draw(assets);
        }

        let pixel_camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);

        let mut player = Player::new();
        player.pos = world.get_interactable_spawn(16).unwrap();

        Self {
            locker_pos: world.get_interactable_spawn(41).unwrap(),
            player,
            assets,
            world,
            pixel_camera,
            world_camera_bg,
            world_camera_fg,
            enemies: Vec::with_capacity(10), // todo: adjust capcacity later on?
            stars: StarsBackground::new(),
        }
    }
    fn update(&mut self) {
        // cap delta time to a minimum of 60 fps.
        let delta_time = get_frame_time().min(1.0 / 60.0);
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_x = mouse_x / scale_factor;
        let mouse_y = mouse_y / scale_factor;

        self.player
            .update(delta_time, &mut self.world, &mut self.enemies);
        self.pixel_camera.target = self.player.camera_pos.floor();
        set_camera(&self.pixel_camera);
        clear_background(BLACK);
        self.stars.draw(delta_time, self.player.camera_pos);

        // draw world texture
        draw_texture_ex(
            &self.world_camera_bg.render_target.as_ref().unwrap().texture,
            (self.world.x_min * 16) as f32,
            (self.world.y_min * 16) as f32,
            WHITE,
            DrawTextureParams::default(),
        );
        let mut can_take_weapon = false;
        if (self.player.pos + vec2(-8.0, 8.0)).distance_squared(self.locker_pos) < 512.0 {
            draw_texture_ex(
                &self.assets.open_locker,
                self.locker_pos.x,
                self.locker_pos.y - self.assets.open_locker.height() + 16.0,
                WHITE,
                DrawTextureParams::default(),
            );
            if self.player.weapon.is_none() {
                can_take_weapon = true;
                draw_texture_ex(
                    &self.assets.weapons.get_at_time(0),
                    self.locker_pos.x + 8.0 + 1.0,
                    self.locker_pos.y - 10.0,
                    WHITE,
                    DrawTextureParams::default(),
                );
            }
        }
        for ((x, y), entity) in self.world.tile_entities.iter_mut() {
            let pos = vec2(*x as f32, *y as f32) * 16.0;
            (entity.draw)(entity, self.assets, pos);
        }
        for enemy in self.enemies.iter_mut() {
            enemy.update(delta_time, &mut self.player, &self.world);
            enemy.draw(self.assets);
        }
        self.player.draw(self.assets, mouse_x, mouse_y);
        draw_texture_ex(
            &self.world_camera_fg.render_target.as_ref().unwrap().texture,
            (self.world.x_min * 16) as f32,
            (self.world.y_min * 16) as f32,
            WHITE,
            DrawTextureParams::default(),
        );
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &self.pixel_camera.render_target.as_ref().unwrap().texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    SCREEN_WIDTH * scale_factor,
                    SCREEN_HEIGHT * scale_factor,
                )),
                ..Default::default()
            },
        );
        if can_take_weapon {
            ui::draw_tooltip(self.assets);
            if is_key_pressed(KeyCode::E) {
                self.player.weapon = Some(Weapon { sprite_index: 0 })
            }
        }
    }
}
#[macroquad::main("space splatter")]
async fn main() {
    let assets = Assets::default();
    let mut game = Game::new(&assets);
    loop {
        game.update();
        next_frame().await
    }
}
