

use bevy::render::camera::ScalingMode;
use bevy::prelude::*;

#[derive(Component)]
pub struct CameraMarker;

pub fn setup(mut commands: Commands) {
        let mut camera_bundle = Camera2dBundle::default();

        camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
                width: (1600.0),
                height: (900.0),
        };
        camera_bundle.projection.near = 1000.;
        camera_bundle.projection.far = -1000.;

        commands.spawn((camera_bundle, CameraMarker));
}