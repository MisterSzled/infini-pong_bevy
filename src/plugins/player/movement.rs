use bevy::prelude::*;

use crate::components::collideable::Collideable;
use crate::components::player::Player;

use crate::events::movement::movement_request::MovementRequest;
use crate::resources::player::player_movement::PlayerAvailableMovement;

pub fn move_player(
        mut player_query: Query<(&Player, &Collideable, &mut Transform)>,
        mut collidables_query: Query<(&Collideable, &Transform), Without<Player>>,
        mut movement_request_event: EventReader<MovementRequest>,
) {
        for movement_request in movement_request_event.read() {
                let (_, player_box, mut transform) = player_query.single_mut();

                let mut player_available_directions = PlayerAvailableMovement::default();

                let player_pos = transform.translation;

                let colliding_directions = player_box.get_colliding_sides(
                        player_pos,
                        collidables_query
                                .transmute_lens::<(&Collideable, &Transform)>()
                                .query(),
                );

                if colliding_directions.up {
                        player_available_directions.can_move_up = false;
                }
                if colliding_directions.down {
                        player_available_directions.can_move_down = false;
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
