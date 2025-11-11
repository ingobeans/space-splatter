use asefile::{self, AsepriteFile};
use image::EncodableLayout;
use macroquad::prelude::*;

pub struct Assets {
    pub tileset: Spritesheet,
}
impl Default for Assets {
    fn default() -> Self {
        Self {
            tileset: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/tileset.ase"), None),
                16.0,
            ),
        }
    }
}

pub struct World {
    pub collision: Vec<Chunk>,
    pub details: Vec<Chunk>,
    pub background: Vec<Chunk>,
    pub interactable: Vec<Chunk>,

    pub x_min: i16,
    pub x_max: i16,
    pub y_min: i16,
    pub y_max: i16,
}
#[expect(dead_code)]
impl World {
    pub fn get_interactable_spawn(&self, tile_index: i16) -> Option<Vec2> {
        for chunk in self.interactable.iter() {
            for (i, tile) in chunk.tiles.iter().enumerate() {
                if *tile == tile_index + 1 {
                    return Some(Vec2::new(
                        (i as i16 % 16 + chunk.x) as f32 * 16.0,
                        (i as i16 / 16 + chunk.y) as f32 * 16.0,
                    ));
                }
            }
        }
        None
    }
    pub fn set_collision_tile(&mut self, x: i16, y: i16, tile: i16) {
        let cx = ((x as f32 / 16.0).floor() * 16.0) as i16;
        let cy = ((y as f32 / 16.0).floor() * 16.0) as i16;

        let chunk = self
            .collision
            .iter_mut()
            .find(|f| f.x == cx && f.y == cy)
            .unwrap();
        chunk.tiles[(x - chunk.x + (y - chunk.y) * 16) as usize] = tile;
    }
}
impl Default for World {
    fn default() -> Self {
        let xml = include_str!("../assets/station.tmx");
        let collision = get_layer(xml, "Collision");
        let detail = get_layer(xml, "Detail");
        let interactable = get_layer(xml, "Interactable");
        let background = get_layer(xml, "Background");
        let mut world = World {
            collision: get_all_chunks(collision),
            details: get_all_chunks(detail),
            background: get_all_chunks(background),
            interactable: get_all_chunks(interactable),
            x_min: 999,
            y_min: 999,
            y_max: -999,
            x_max: -999,
        };

        // define x y min and max
        for layer in [
            &world.collision,
            &world.details,
            &world.background,
            &world.interactable,
        ] {
            for chunk in layer {
                if chunk.x < world.x_min {
                    world.x_min = chunk.x;
                }
                if chunk.y < world.y_min {
                    world.y_min = chunk.y;
                }
                if chunk.x > world.x_max {
                    world.x_max = chunk.x;
                }
                if chunk.y > world.y_max {
                    world.y_max = chunk.y;
                }
            }
        }

        world
    }
}
pub struct Chunk {
    pub x: i16,
    pub y: i16,
    pub tiles: Vec<i16>,
}
impl Chunk {
    pub fn tile_at(&self, x: usize, y: usize) -> Option<i16> {
        if x > 16 {
            return None;
        }
        self.tiles.get(x + y * 16).cloned()
    }
    pub fn draw(&self, assets: &Assets) {
        for (index, tile) in self.tiles.iter().enumerate() {
            if *tile == 0 {
                continue;
            }
            let tile = *tile - 1;
            let x = index % 16;
            let y = index / 16;
            assets.tileset.draw_tile(
                (self.x * 16) as f32 + (x * 16) as f32,
                (self.y * 16) as f32 + (y * 16) as f32,
                (tile % 16) as f32,
                (tile / 16) as f32,
                None,
            );
        }
    }
}
fn get_all_chunks(xml: &str) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut xml = xml.to_string();
    while let Some((current, remains)) = xml.split_once("</chunk>") {
        let new = parse_chunk(current);
        chunks.push(new);
        xml = remains.to_string();
    }

    chunks
}
fn parse_chunk(xml: &str) -> Chunk {
    let (tag, data) = xml
        .split_once("<chunk ")
        .unwrap()
        .1
        .split_once(">")
        .unwrap();

    let x = tag
        .split_once("x=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0
        .parse()
        .unwrap();
    let y = tag
        .split_once("y=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0
        .parse()
        .unwrap();

    let mut split = data.split(',');

    let mut chunk = vec![0; 16 * 16];
    for item in &mut chunk {
        let a = split.next().unwrap().trim();
        *item = a.parse().unwrap()
    }
    Chunk { x, y, tiles: chunk }
}

fn get_layer<'a>(xml: &'a str, layer: &str) -> &'a str {
    let split = format!(" name=\"{layer}");
    xml.split_once(&split)
        .unwrap()
        .1
        .split_once(">")
        .unwrap()
        .1
        .split_once("</layer>")
        .unwrap()
        .0
}

fn load_ase_texture(bytes: &[u8], layer: Option<u32>) -> Texture2D {
    let img = AsepriteFile::read(bytes).unwrap();
    let img = if let Some(layer) = layer {
        img.layer(layer).frame(0).image()
    } else {
        img.frame(0).image()
    };
    let new = Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_bytes().to_vec(),
    };
    let texture = Texture2D::from_image(&new);
    texture.set_filter(FilterMode::Nearest);
    texture
}
pub struct Spritesheet {
    pub texture: Texture2D,
    pub sprite_size: f32,
}
impl Spritesheet {
    pub fn new(texture: Texture2D, sprite_size: f32) -> Self {
        Self {
            texture,
            sprite_size,
        }
    }
    /// Same as `draw_tile`, except centered
    pub fn draw_sprite(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<&DrawTextureParams>,
    ) {
        self.draw_tile(
            screen_x - self.sprite_size / 2.0,
            screen_y - self.sprite_size / 2.0,
            tile_x,
            tile_y,
            params,
        );
    }
    /// Draws a single tile from the spritesheet
    pub fn draw_tile(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<&DrawTextureParams>,
    ) {
        let mut p = params.cloned().unwrap_or(DrawTextureParams::default());
        p.dest_size = p
            .dest_size
            .or(Some(Vec2::new(self.sprite_size, self.sprite_size)));
        p.source = p.source.or(Some(Rect {
            x: tile_x * self.sprite_size,
            y: tile_y * self.sprite_size,
            w: self.sprite_size,
            h: self.sprite_size,
        }));
        draw_texture_ex(&self.texture, screen_x, screen_y, WHITE, p);
    }
}
