use super::*;
use crate::tile::Tile;

pub struct Map {
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Map {
        Map { tiles: Vec::new() }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &Tile {
        self.tiles
            .iter()
            .find(|t| t.x == (x / 32) && t.y == (y / 32))
            .unwrap_or(&Tile {
                x: 1,
                y: 1,
                tile_type: tile::TileType::EMPTY,
            })
    }
}

impl Renderable for Map {
    fn render(&self) {
        for tile in &self.tiles {
            draw_rectangle((tile.x * 32) as f32, (tile.y * 32) as f32, 32.0, 32.0, RED)
        }
    }
    fn render_texture(&self, texture: Texture2D) {
        for tile in &self.tiles {
            draw_texture(texture, (tile.x * 32) as f32, (tile.y * 32) as f32, GREEN)
        }
    }
}

pub trait Renderable {
    fn render(&self);
    fn render_texture(&self, texture: Texture2D);
}
