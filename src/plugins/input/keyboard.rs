use bevy::prelude::*;

use crate::events::movement::movement_request::MovementRequest;

pub fn player_movement(
        mut movement_request_event: EventWriter<MovementRequest>,
        keys: Res<ButtonInput<KeyCode>>,
) {
        let mut dirs = MovementRequest {
                up: false,
                down: false,
                left: false,
                right: false,
        };
        let mut valid = false;

        if keys.pressed(KeyCode::KeyW) {
                dirs.up = true;
                valid = true;
        }
        if keys.pressed(KeyCode::KeyS) {
                dirs.down = true;
                valid = true;
        }
        if keys.pressed(KeyCode::KeyD) {
                dirs.right = true;
                valid = true;
        }
        if keys.pressed(KeyCode::KeyA) {
                dirs.left = true;
                valid = true;
        }

        if valid {
                movement_request_event.send(dirs);
        }
}
