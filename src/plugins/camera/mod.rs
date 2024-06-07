use bevy::prelude::*;

mod init;
mod resize;

pub struct Camera;

impl Plugin for Camera {
        fn build(&self, app: &mut App) {
                app.add_systems(Startup, init::setup);
                app.add_systems(Update, resize::resize);
        }
}
