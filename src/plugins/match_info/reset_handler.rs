use bevy::prelude::*;

use crate::components::match_resetter::MatchResetter;
use crate::components::non_repeating_timer::NonRepeatingTimer;

use crate::components::ball::Ball;
use crate::components::collideable::Collideable;
use crate::components::velocity::Velocity;

pub fn reset_handler(
        mut commands: Commands,
        mut resetter_query: Query<(Entity, &MatchResetter, &mut NonRepeatingTimer)>,
        mut ball_query: Query<(&Ball, &Collideable, &mut Transform, &mut Velocity)>,
        time: Res<Time>,
) {
        for (entity, _, mut non_repeating_timer) in resetter_query.iter_mut() {
                non_repeating_timer.timer.tick(time.delta());

                if non_repeating_timer.timer.finished() {

                        let (_, _, _, mut velocity) = ball_query.single_mut();
                        *velocity = Velocity::new();

                        commands.entity(entity).despawn_recursive();
                }
        }
}
