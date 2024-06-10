use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1600.;
const WINDOW_HEIGHT: f32 = 900.;

const IN_GAME_UI_HEIGHT: f32 = 68.;

#[derive(Resource, Debug, Copy, Clone)]
pub struct Config {
        pub window_width: f32,
        pub window_height: f32,

        pub in_game_ui_height: f32,
}

impl Default for Config {
        fn default() -> Self {
                Config {
                        window_width: WINDOW_WIDTH,
                        window_height: WINDOW_HEIGHT,

                        in_game_ui_height: IN_GAME_UI_HEIGHT,
                }
        }
}