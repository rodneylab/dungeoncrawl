use bevy_ecs::query::With;
use bevy_ecs::system::{Query, Res};

use crate::prelude::*;
use crate::systems::idx as map_idx;

#[allow(clippy::needless_pass_by_value)]
pub fn map_render(
    player_fov_query: Query<&FieldOfView, With<Player>>,
    (map, camera, theme): (Res<Map>, Res<Camera>, Res<Box<dyn MapTheme>>),
) {
    //let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player_fov = player_fov_query.single();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx]
            {
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };
                let glyph = theme.tile_to_render(map.tiles[idx]);
                draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
