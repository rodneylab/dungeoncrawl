use bracket_lib::terminal::Point;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let Point { x, y } = player_position;
        Self {
            left_x: x - DISPLAY_WIDTH / 2,
            right_x: x + DISPLAY_WIDTH / 2,
            top_y: y - DISPLAY_HEIGHT / 2,
            bottom_y: y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        let Point { x, y } = player_position;
        self.left_x = x - DISPLAY_WIDTH / 2;
        self.right_x = x + DISPLAY_WIDTH / 2;
        self.top_y = y - DISPLAY_HEIGHT / 2;
        self.bottom_y = y + DISPLAY_HEIGHT / 2;
    }
}
