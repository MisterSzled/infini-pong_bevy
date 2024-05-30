use bevy::prelude::*;

mod initiate;

pub struct Enemy;

impl Plugin for Enemy {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, initiate::setup);
        }
}
