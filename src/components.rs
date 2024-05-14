use bevy_ecs::{component::Component, entity::Entity};
use bracket_lib::{
    color::ColorPair,
    terminal::{FontCharType, Point},
};

use std::collections::HashSet;

#[derive(Component)]
pub struct PointC(pub Point);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Component)]
pub struct Carried(pub Entity);

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Item;

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Player {
    pub map_level: u32,
}

#[derive(Component)]
pub struct ProvidesDungeonMap;

#[derive(Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Component)]
pub struct Weapon;
