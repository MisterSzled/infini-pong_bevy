use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;
use std::cmp;

pub fn resize(
        mut resize_reader: EventReader<WindowResized>,
        mut camera_query: Query<&mut Camera>,
        window: Query<&Window>,
) {
        for _e in resize_reader.read() {
                let window = window.single();

                let window_w = window.resolution.physical_width();
                let window_h = window.resolution.physical_height();

                let height_count = window_h / 9;
                let width_count = window_w / 16;

                let rec_count = cmp::min(height_count, width_count);

                let target_height = rec_count * 9;
                let target_width = rec_count * 16;

                let draw_x = (window_w - target_width) / 2;
                let draw_y = (window_h - target_height) / 2;

                for mut camera in camera_query.iter_mut() {
                        camera.viewport = Some(Viewport {
                                physical_position: UVec2::new(draw_x, draw_y),
                                physical_size: UVec2::new(target_width, target_height),
                                ..default()
                        });
                }
        }
}
