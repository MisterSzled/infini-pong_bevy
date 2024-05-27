use bevy::prelude::*;

mod initiate;

pub struct Player;

impl Plugin for Player {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, initiate::setup);
        }
}
