#![warn(clippy::all, clippy::pedantic)]

mod camera;
mod components;
mod events;
mod game_stage;
mod map;
mod map_builder;
mod spawner;
mod state_label;
mod systems;
mod turn_state;

use bevy_app::{App, CoreStage};
use bevy_ecs::{entity::Entity, prelude::SystemStage, query::With};
use bracket_lib::{
    color::{BLACK, GREEN, RED, WHITE, YELLOW},
    pathfinding::Algorithm2D,
    random::RandomNumberGenerator,
    terminal::{
        main_loop, render_draw_buffer, BError, BTerm, BTermBuilder, GameState, Point,
        VirtualKeyCode,
    },
};
use spawner::{spawn_amulet_of_yala, spawn_level, spawn_player};
mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
}
use crate::{
    camera::Camera,
    components::{Carried, FieldOfView, Player, PointC, WantsToAttack, WantsToMove},
    events::ActivateItem,
    systems::build_system_sets,
    turn_state::TurnState,
};
pub use map::{Map, TileType};
pub use map_builder::MapBuilder;
use std::collections::HashSet;

struct State {
    ecs: App,
}

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

impl State {
    fn new() -> Self {
        use game_stage::GameStage::{
            GenerateMonsterMoves, MonsterCombat, MonsterFov, MoveMonsters, MovePlayer,
            PlayerCombat, PlayerFov,
        };

        let mut ecs = App::new();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs.world, map_builder.player_start);
        //spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&mut ecs.world, &mut rng, 0, &map_builder.monster_spawns);
        ecs.insert_resource(map_builder.map);
        ecs.insert_resource(Camera::new(map_builder.player_start));
        ecs.add_event::<WantsToMove>();
        ecs.add_event::<WantsToAttack>();
        ecs.add_event::<ActivateItem>();
        ecs.add_stage_after(CoreStage::Update, PlayerCombat, SystemStage::parallel())
            .add_stage_after(PlayerCombat, MovePlayer, SystemStage::parallel())
            .add_stage_after(MovePlayer, PlayerFov, SystemStage::parallel())
            .add_stage_after(PlayerFov, GenerateMonsterMoves, SystemStage::parallel())
            .add_stage_after(GenerateMonsterMoves, MonsterCombat, SystemStage::parallel())
            .add_stage_after(MonsterCombat, MoveMonsters, SystemStage::parallel())
            .add_stage_after(MoveMonsters, MonsterFov, SystemStage::parallel());
        ecs.insert_resource(TurnState::AwaitingInput);
        ecs.insert_resource(map_builder.theme);
        build_system_sets(&mut ecs);
        Self { ecs }
    }

    fn advance_level(&mut self) {
        let mut player_query = self.ecs.world.query_filtered::<Entity, With<Player>>();
        let player_entity = player_query.iter(&self.ecs.world).next().unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        let mut carry_query = self.ecs.world.query::<(Entity, &Carried)>();
        for (e, carry) in carry_query.iter(&self.ecs.world) {
            if carry.0 == player_entity {
                entities_to_keep.insert(e);
            }
        }
        let mut entities_query = self.ecs.world.query::<Entity>();
        let entities_to_remove = entities_query
            .iter(&self.ecs.world)
            .filter(|e| (!entities_to_keep.contains(e)))
            .collect::<Vec<_>>();
        for e in entities_to_remove {
            self.ecs.world.despawn(e);
        }

        let mut fov_query = self.ecs.world.query::<&mut FieldOfView>();
        for mut fov in fov_query.iter_mut(&mut self.ecs.world) {
            fov.is_dirty = true;
        }

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        let mut player_query = self.ecs.world.query::<(&mut Player, &mut PointC)>();
        for (mut player, mut pos) in player_query.iter_mut(&mut self.ecs.world) {
            player.map_level += 1;
            map_level = player.map_level;
            pos.0.x = map_builder.player_start.x;
            pos.0.y = map_builder.player_start.y;
        }

        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs.world, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }
        spawn_level(
            &mut self.ecs.world,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );
        self.ecs.world.insert_resource(map_builder.map);
        self.ecs
            .world
            .insert_resource(Camera::new(map_builder.player_start));
        self.ecs.world.insert_resource(TurnState::AwaitingInput);
        self.ecs.world.insert_resource(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs.world.clear_entities();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs.world, map_builder.player_start);
        //spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        //map_builder.map.point2d_to_index(map_builder.amulet_start);
        //        map_builder
        //            .monster_spawns
        //            .iter()
        //            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rng, *pos));
        spawn_level(
            &mut self.ecs.world,
            &mut rng,
            0,
            &map_builder.monster_spawns,
        );
        self.ecs.insert_resource(map_builder.map);
        self.ecs
            .insert_resource(Camera::new(map_builder.player_start));
        self.ecs.insert_resource(TurnState::AwaitingInput);
        self.ecs.insert_resource(map_builder.theme);
        self.ecs.world.remove_resource::<VirtualKeyCode>();
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        if let Some(key) = ctx.key {
            self.ecs.insert_resource(key);
        } else {
            self.ecs.world.remove_resource::<VirtualKeyCode>();
        }
        ctx.set_active_console(0);
        self.ecs.insert_resource(Point::from_tuple(ctx.mouse_pos()));
        match self.ecs.world.get_resource::<TurnState>() {
            Some(TurnState::NextLevel) => {
                self.advance_level();
            }
            Some(TurnState::GameOver) => {
                self.game_over(ctx);
            }
            Some(TurnState::Victory) => self.victory(ctx),
            _ => {}
        }
        self.ecs.update();
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
