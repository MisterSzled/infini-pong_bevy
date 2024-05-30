use bevy::prelude::*;

mod initiate;
mod controller;

pub struct Ball;

impl Plugin for Ball {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, initiate::setup);
                app.add_systems(Update, controller::update_controller);
        }
}
