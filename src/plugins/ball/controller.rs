use bevy::prelude::*;

use crate::components::ball::Ball;
use crate::components::collideable::Collideable;
use crate::components::velocity::Velocity;
use crate::events::score::increment_score_event::IncrementScoreEvent;
use crate::events::score::reset_ball_event::ResetBallEvent;

pub fn update_controller(
        mut ball_query: Query<(&Ball, &Collideable, &mut Transform, &mut Velocity)>,
        mut collidables_query: Query<(&Collideable, &Transform), Without<Ball>>,
        mut score_change_emitter: EventWriter<IncrementScoreEvent>,
        mut reset_ball_emitter: EventWriter<ResetBallEvent>,
) {
        let (_, ball_box, mut transform, mut velocity) = ball_query.single_mut();
        let ball_pos = transform.translation;

        // Check for score change and reset if true
        if transform.translation.x > 800. {
                score_change_emitter.send(IncrementScoreEvent {
                        player: true,
                        enemy: false,
                });
                reset_ball_emitter.send(ResetBallEvent);
        }
        if transform.translation.x < -800. {
                score_change_emitter.send(IncrementScoreEvent {
                        player: false,
                        enemy: true,
                });
                reset_ball_emitter.send(ResetBallEvent);
        }


        // handle collisions
        let colliding_directions = ball_box.get_colliding_sides(
                ball_pos,
                collidables_query
                        .transmute_lens::<(&Collideable, &Transform)>()
                        .query(),
        );

        if colliding_directions.up || colliding_directions.down {
                velocity.y *= -1.;
        }
        if colliding_directions.left || colliding_directions.right {
                velocity.x *= -1.;
        }

        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
}
