use bracket_lib::{
    pathfinding::{Algorithm2D, DijkstraMap},
    random::RandomNumberGenerator,
    terminal::Point,
};

use crate::{Map, MapBuilder, TileType, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::MapArchitect;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;
const STAGGER_DISTANCE: usize = 400;

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

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        walker(center, rng, &mut mb.map);
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            walker(
                Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &[mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

fn walker(start: Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
    let mut walker_pos = start;
    let mut distance_staggered = 0;

    loop {
        let random_idx = map.point2d_to_index(walker_pos);
        map.tiles[random_idx] = TileType::Floor;

        match rng.range(0, 4) {
            0 => walker_pos.x -= 1,
            1 => walker_pos.x += 1,
            2 => walker_pos.y -= 1,
            3 => walker_pos.y += 1,
            _ => unreachable!("Random number in (0..4] range expected"),
        }
        if !map.in_bounds(walker_pos) {
            break;
        }

        distance_staggered += 1;
        if distance_staggered > STAGGER_DISTANCE {
            break;
        }
    }
}
