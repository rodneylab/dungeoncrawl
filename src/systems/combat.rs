use crate::components;
use bevy_ecs::{
    event::EventReader,
    system::{Commands, Query},
};
use components::{Carried, Damage, Health, Player, WantsToAttack};

#[allow(clippy::needless_pass_by_value)]
pub fn combat(
    mut commands: Commands,
    mut attack_events: EventReader<WantsToAttack>,
    mut health_query: Query<&mut Health>,
    player_query: Query<&Player>,
    base_damage_query: Query<&Damage>,
    carried_weapons_query: Query<(&Carried, &Damage)>,
) {
    for WantsToAttack { attacker, victim } in attack_events.iter() {
        let is_player = player_query.get(*victim).is_ok();

        let base_damage = if let Ok(dmg) = base_damage_query.get(*attacker) {
            dmg.0
        } else {
            0
        };

        let weapon_damage: i32 = carried_weapons_query
            .iter()
            .filter_map(|(carried, dmg)| (carried.0 == *attacker).then_some(dmg.0))
            .sum();

        let final_damage = base_damage + weapon_damage;

        if let Ok(mut health) = health_query.get_mut(*victim) {
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                commands.entity(*victim).despawn();
            }
        }
    }
}
