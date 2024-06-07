use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::camera::Viewport;

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

        camera_bundle.camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(1600, 900),
                ..default()
        });

        camera_bundle.camera.clear_color = ClearColorConfig::Custom(Color::rgb(0., 0., 0.));

        commands.spawn((Name::new("Main_Camera"), CameraMarker, camera_bundle));
}
