use bevy_ecs::entity::Entity;

pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
