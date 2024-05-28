use bevy::prelude::*;

mod initiate;
mod resize;

pub struct Camera;

impl Plugin for Camera {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, initiate::setup);
                app.add_systems(Update, resize::resize);
        }
}
