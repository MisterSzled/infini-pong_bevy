use bevy::prelude::*;

mod player_movement;

pub struct Input;

impl Plugin for Input {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, player_movement::initiate);
        }
}
