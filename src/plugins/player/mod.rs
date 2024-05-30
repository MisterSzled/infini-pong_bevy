use bevy::prelude::*;

mod initiate;
mod movement;

pub struct Player;



impl Plugin for Player {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, initiate::setup);
                app.add_systems(Update, movement::move_player);
        }
}
