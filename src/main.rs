use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{assets::*, player::*, utils::*};

mod assets;
mod player;
mod utils;

struct Game<'a> {
    assets: &'a Assets,
    world: World,
    player: Player,
    pixel_camera: Camera2D,
    world_camera_bg: Camera2D,
    world_camera_fg: Camera2D,
    stars: StarsBackground,
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
            player,
            assets,
            world,
            pixel_camera,
            world_camera_bg,
            world_camera_fg,
            stars: StarsBackground::new(),
        }
    }
    fn update(&mut self) {
        // cap delta time to a minimum of 60 fps.
        let delta_time = get_frame_time().min(1.0 / 60.0);
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);

        self.player.update(delta_time, &self.world);
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
        self.player.draw(self.assets);
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
    }
}
#[macroquad::main("space friend")]
async fn main() {
    let assets = Assets::default();
    let mut game = Game::new(&assets);
    loop {
        game.update();
        next_frame().await
    }
}
