use bevy::prelude::*;

use std::time::Duration;

use crate::components::ball::Ball;
use crate::components::collideable::Collideable;
use crate::components::match_resetter::MatchResetter;
use crate::components::non_repeating_timer::NonRepeatingTimer;
use crate::components::velocity::Velocity;
use crate::events::score::reset_ball_event::ResetBallEvent;

pub fn reset_ball(
        mut commands: Commands,
        mut ball_query: Query<(&Ball, &Collideable, &mut Transform, &mut Velocity)>,
        mut reset_ball_event: EventReader<ResetBallEvent>,
) {
        for _ in reset_ball_event.read() {
                let (_, _, mut transform, mut velocity) = ball_query.single_mut();
                transform.translation.x = 0.;
                transform.translation.y = 0.;

                velocity.x = 0.;
                velocity.y = 0.;

                commands.spawn((
                        Name::new("Resetter"),
                        MatchResetter,
                        NonRepeatingTimer {
                                timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
                        },
                ));
        }
}
