use bevy::prelude::*;
use crate::AppStates;

mod keyboard;

pub struct Input;

impl Plugin for Input {
        fn build(&self, app: &mut App) {

                app.add_systems(
                        Update,
                        keyboard::player_movement.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
        }
}
