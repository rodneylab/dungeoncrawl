use bracket_lib::{
    pathfinding::Algorithm2D,
    random::RandomNumberGenerator,
    terminal::{DistanceAlg, Point},
};

use super::MapArchitect;
use crate::{map::idx as map_idx, Map, MapBuilder, TileType, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Architect {}

impl MapArchitect for Architect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };
        random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            Self::iteration(&mut mb.map);
        }
        let start = Self::find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl Architect {
    fn count_neighbours(x: i32, y: i32, map: &Map) -> usize {
        let mut neighbours = 0;
        // spellchecker:off
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    // spellchecker:on
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    fn find_start(map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point)
    }

    fn iteration(map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbours = Self::count_neighbours(x, y, map);
                let idx = map_idx(x, y);
                if neighbours > 4 || neighbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }
}

fn random_noise_map(rng: &mut RandomNumberGenerator, map: &mut Map) {
    map.tiles.iter_mut().for_each(|t| {
        let roll = rng.range(0, 100);
        if roll > 55 {
            *t = TileType::Floor;
        } else {
            *t = TileType::Wall;
        }
    });
}
