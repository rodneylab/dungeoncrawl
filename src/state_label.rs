use bevy_ecs::schedule::SystemLabel;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum StateLabel {
    Fov,
}
