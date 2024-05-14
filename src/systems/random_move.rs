use crate::components::{Health, MovingRandomly, Player, PointC, WantsToAttack, WantsToMove};
use bevy_ecs::{entity::Entity, event::EventWriter, query::With, system::Query};
use bracket_lib::{random::RandomNumberGenerator, terminal::Point};

#[allow(clippy::needless_pass_by_value)]
pub fn random_move(
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    movers: Query<(Entity, &PointC), With<MovingRandomly>>,
    positions: Query<(Entity, &PointC), With<Health>>,
    player_query: Query<Entity, With<Player>>,
) {
    movers.iter().for_each(|(entity, pos)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + pos.0;
        let mut attacked = false;

        for (victim, target_pos) in positions.iter() {
            if target_pos.0 == destination {
                if player_query.get(victim).is_ok() {
                    attack_events.send(WantsToAttack {
                        attacker: entity,
                        victim,
                    });
                }

                attacked = true;
            }
        }
        if !attacked {
            move_events.send(WantsToMove {
                entity,
                destination,
            });
        }
    });
}
