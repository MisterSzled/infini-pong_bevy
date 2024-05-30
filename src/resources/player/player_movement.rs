use bevy::prelude::*;

#[derive(Resource, Debug, Copy, Clone)]
pub struct PlayerAvailableMovement {
        // pub can_move_left: bool,
        // pub can_move_right: bool,
        pub can_move_up: bool,
        pub can_move_down: bool,
}

impl Default for PlayerAvailableMovement {
        fn default() -> Self {
                PlayerAvailableMovement {
                        // can_move_left: true,
                        // can_move_right: true,
                        can_move_up: true,
                        can_move_down: true,
                }
        }
}
