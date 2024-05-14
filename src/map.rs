use bracket_lib::{
    pathfinding::{Algorithm2D, BaseMap, SmallVec},
    terminal::{DistanceAlg, Point},
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

#[allow(clippy::cast_sign_loss)]
pub fn idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[must_use]
    pub fn in_bounds(&self, point: Point) -> bool {
        let Point { x, y } = point;
        (0..SCREEN_WIDTH).contains(&x) && (0..SCREEN_HEIGHT).contains(&y)
    }

    #[must_use]
    pub fn can_enter_tile(&self, point: Point) -> bool {
        Self::in_bounds(self, point)
            && (self.tiles[idx(point.x, point.y)] == TileType::Floor
                || self.tiles[idx(point.x, point.y)] == TileType::Exit)
    }

    #[must_use]
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if Self::in_bounds(self, point) {
            Some(idx(point.x, point.y))
        } else {
            None
        }
    }
}
