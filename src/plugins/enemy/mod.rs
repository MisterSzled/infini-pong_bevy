use bevy::prelude::*;

mod controller;
mod initiate;

pub struct Enemy;

impl Plugin for Enemy {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, initiate::setup);
                app.add_systems(Update, controller::update_controller);
        }
}
