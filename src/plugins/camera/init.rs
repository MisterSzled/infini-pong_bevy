use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::camera::Viewport;

use crate::resources::config::config::Config;

#[derive(Component)]
pub struct CameraMarker;

pub fn setup(
        mut commands: Commands,
        config: Res<Config>
) {
        let mut camera_bundle = Camera2dBundle::default();

        camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
                width: (config.window_width),
                height: (config.window_height),
        };
        camera_bundle.projection.near = 1000.;
        camera_bundle.projection.far = -1000.;

        camera_bundle.camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(config.window_width as u32, config.window_height as u32),
                ..default()
        });

        camera_bundle.camera.clear_color = ClearColorConfig::Custom(Color::rgb(0., 0., 0.));

        commands.spawn((Name::new("Main_Camera"), CameraMarker, camera_bundle));
}
