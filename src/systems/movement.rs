use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, ResMut};

use crate::map::idx as map_idx;
use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn movement(
    mut commands: Commands,
    mut move_events: EventReader<WantsToMove>,
    query: Query<(Entity, &FieldOfView, Option<&Player>)>,
    (mut map, mut camera): (ResMut<Map>, ResMut<Camera>),
) {
    for &WantsToMove {
        entity,
        destination,
    } in move_events.iter()
    {
        if map.can_enter_tile(destination) {
            commands.entity(entity).insert(PointC(destination));

            if let Ok((entity, fov, player)) = query.get(entity) {
                commands.entity(entity).insert(fov.clone_dirty());

                if player.is_some() {
                    camera.on_player_move(destination);

                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
}
