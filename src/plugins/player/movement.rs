use bevy::{
        math::bounding::{Aabb2d, IntersectsVolume, RayCast2d},
        math::primitives::Direction2d,
        prelude::*,
};

use crate::components::collideable::Collideable;
use crate::components::player::Player;

use crate::events::movement::movement_request::MovementRequest;

use crate::resources::player::player_movement::PlayerAvailableMovement;

pub fn move_player(
        mut player_query: Query<(&Player, &Collideable, &mut Transform)>,
        collidables_query: Query<(&Collideable, &Transform), Without<Player>>,
        mut movement_request_event: EventReader<MovementRequest>,
) {
        for movement_request in movement_request_event.read() {
                let (player, player_box, mut transform) = player_query.single_mut();

                let mut player_available_directions = PlayerAvailableMovement::default();

                let player_pos = transform.translation;

                let player_aabb = Aabb2d {
                        min: Vec2 {
                                x: player_pos.x - (player_box.width / 2.),
                                y: player_pos.y - (player_box.height / 2.),
                        },
                        max: Vec2 {
                                x: player_pos.x + (player_box.width / 2.),
                                y: player_pos.y + (player_box.height / 2.),
                        },
                };

                for (collidable, transform) in collidables_query.iter() {
                        let center = transform.translation;

                        let collidable_aabb = Aabb2d {
                                min: Vec2 {
                                        x: center.x - (collidable.width / 2.),
                                        y: center.y - (collidable.height / 2.),
                                },
                                max: Vec2 {
                                        x: center.x + (collidable.width / 2.),
                                        y: center.y + (collidable.height / 2.),
                                },
                        };

                        if player_aabb.intersects(&collidable_aabb) {
                                let pos_y_ray = RayCast2d::new(
                                        Vec2 {
                                                x: player_pos.x,
                                                y: player_pos.y,
                                        },
                                        Direction2d::Y,
                                        50.,
                                );
                                let neg_y_ray = RayCast2d::new(
                                        Vec2 {
                                                x: player_pos.x,
                                                y: player_pos.y,
                                        },
                                        Direction2d::NEG_Y,
                                        50.,
                                );

                                let pos_y_intersection =
                                        pos_y_ray.aabb_intersection_at(&collidable_aabb);
                                let neg_y_intersection =
                                        neg_y_ray.aabb_intersection_at(&collidable_aabb);

                                if Option::is_some(&pos_y_intersection) {
                                        player_available_directions.can_move_up = false
                                }
                                if Option::is_some(&neg_y_intersection) {
                                        player_available_directions.can_move_down = false
                                }
                        }
                }

                let mut delta = Vec3::new(0., 0., 0.);

                if movement_request.up && player_available_directions.can_move_up {
                        delta.y += 1.;
                }
                if movement_request.down && player_available_directions.can_move_down {
                        delta.y -= 1.;
                }

                transform.translation += delta;
        }
}
