use super::MapTheme;
use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl ForestTheme {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
