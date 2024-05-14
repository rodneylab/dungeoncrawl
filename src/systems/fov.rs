use crate::{components, Map};
use bevy_ecs::system::{Query, Res};
use bracket_lib::pathfinding::field_of_view_set;
use components::{FieldOfView, PointC};

#[allow(clippy::needless_pass_by_value)]
pub fn fov(mut views: Query<(&PointC, &mut FieldOfView)>, map: Res<Map>) {
    for (pos, mut fov) in views.iter_mut() {
        if fov.is_dirty {
            fov.visible_tiles = field_of_view_set(pos.0, fov.radius, map.as_ref());
            fov.is_dirty = false;
        }
    }
}
