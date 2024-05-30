use bevy::{
        prelude::*,
        // sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::components::ball::Ball;
use crate::components::collideable::Collideable;
use crate::components::velocity::Velocity;

pub fn update_controller(
        mut ball_query: Query<(&Ball, &Collideable, &mut Transform, &mut Velocity)>,
        mut collidables_query: Query<(&Collideable, &Transform), Without<Ball>>,
) {
        let (_, ball_box, mut transform, mut velocity) = ball_query.single_mut();

        let ball_pos = transform.translation;

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
