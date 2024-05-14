use crate::{components, SCREEN_WIDTH};
use bevy_ecs::{entity::Entity, query::With, system::Query};
use bracket_lib::{
    color::{ColorPair, BLACK, RED, WHITE, YELLOW},
    terminal::{DrawBatch, Point},
};
use components::{Carried, Health, Item, Name, Player};

#[allow(clippy::needless_pass_by_value)]
pub fn hud(
    player_query: Query<(Entity, &Player, &Health)>,
    item_query: Query<(&Name, &Carried), With<Item>>,
) {
    let (player_entity, player, player_health) = player_query.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", player.map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut y = 3;
    for (name, carried) in item_query.iter() {
        if carried.0 == player_entity {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        };
    }
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(100_000).expect("Batch error");
}
