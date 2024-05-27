use bevy::prelude::*;

mod initiate;

pub struct Camera;

impl Plugin for Camera {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, initiate::setup);
        }
}
