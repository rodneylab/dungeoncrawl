use crate::{components, map, turn_state};
use bevy_ecs::{
    query::With,
    system::{Commands, Query, Res},
};
use bracket_lib::{pathfinding::Algorithm2D, terminal::Point};
use components::{AmuletOfYala, Health, Player, PointC};
use map::{Map, TileType};
use turn_state::TurnState;

#[allow(clippy::needless_pass_by_value)]
pub fn end_turn(
    mut commands: Commands,
    player_query: Query<(&Health, &PointC), With<Player>>,
    amulet_query: Query<&PointC, With<AmuletOfYala>>,
    turn_state: Res<TurnState>,
    map: Res<Map>,
) {
    let (player_hp, player_pos) = player_query.single();
    let mut new_state = match *turn_state {
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::AwaitingInput
        | TurnState::NextLevel
        | TurnState::GameOver
        | TurnState::Victory => unreachable!(),
    };

    let amulet_default = PointC(Point::new(-1, -1));
    let amulet_pos = amulet_query.get_single().unwrap_or(&amulet_default);

    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }
    if player_pos.0 == amulet_pos.0 {
        new_state = TurnState::Victory;
    }
    let idx = map.point2d_to_index(player_pos.0);
    if map.tiles[idx] == TileType::Exit {
        new_state = TurnState::NextLevel;
    }

    commands.insert_resource(new_state);
}
