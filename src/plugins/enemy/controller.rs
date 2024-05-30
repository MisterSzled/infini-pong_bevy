use bevy::prelude::*;

use crate::components::collideable::Collideable;
use crate::components::enemy::Enemy;
use crate::components::ball::Ball;

pub fn update_controller(
        ball_query: Query<(&Ball, &Transform)>,
        mut enemy_query: Query<(&Enemy, &Collideable, &mut Transform), Without<Ball>>,
        mut collidables_query: Query<(&Collideable, &Transform), Without<Enemy>>,
) {
        let (_, enemy_box, mut transform) = enemy_query.single_mut();
        let (_, ball_transform) = ball_query.single();

        let enemy_pos = transform.translation;

        let colliding_directions = enemy_box.get_colliding_sides(
                enemy_pos,
                collidables_query
                        .transmute_lens::<(&Collideable, &Transform)>()
                        .query(),
        );

        let mut delta = Vec3::new(0., 0., 0.);

        if !colliding_directions.up && ball_transform.translation.y > enemy_pos.y {
                delta.y += 2.;
        }
        if !colliding_directions.down && ball_transform.translation.y < enemy_pos.y {
                delta.y -= 2.;
        }

        transform.translation += delta;
}