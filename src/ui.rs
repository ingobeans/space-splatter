use crate::assets::Assets;
use crate::player::Player;
use crate::utils::*;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

pub const PLAYER_HEALTH_COLOR: Color = Color::from_hex(0x87d1ef);

pub fn draw_ui(
    assets: &Assets,
    player: &Player,
    show_item_tooltip: bool,
    show_escape_tooltip: bool,
) {
    let (actual_screen_width, actual_screen_height) = screen_size();
    let scale_factor = (actual_screen_width / SCREEN_WIDTH)
        .min(actual_screen_height / SCREEN_HEIGHT)
        .floor()
        .max(1.0);

    let x = 10.0 * scale_factor;
    let y = 10.0 * scale_factor;
    draw_rectangle(
        x + 8.0 * scale_factor,
        y + 2.0 * scale_factor,
        170.0 * scale_factor * player.health / 100.0,
        20.0 * scale_factor,
        BLACK,
    );
    draw_rectangle(
        x + 8.0 * scale_factor,
        y + 2.0 * scale_factor,
        170.0 * scale_factor * player.health / 100.0,
        20.0 * scale_factor,
        PLAYER_HEALTH_COLOR,
    );
    draw_texture_ex(
        &assets.healthbar,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                assets.tooltip.width() * scale_factor,
                assets.tooltip.height() * scale_factor,
            )),
            ..Default::default()
        },
    );

    let tooltip = if show_item_tooltip {
        Some(&assets.tooltip)
    } else if show_escape_tooltip {
        Some(&assets.escape_pod_tooltip)
    } else {
        None
    };
    if let Some(tooltip) = tooltip {
        let x = (actual_screen_width - tooltip.width() * scale_factor) / 2.0;
        let y = actual_screen_height - tooltip.height() * scale_factor - 4.0 * scale_factor;
        draw_texture_ex(
            &tooltip,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    tooltip.width() * scale_factor,
                    tooltip.height() * scale_factor,
                )),
                ..Default::default()
            },
        );
    }
}
