use bevy::prelude::*;

mod plugins;

fn main() {
        App::new()
                .add_plugins(DefaultPlugins)
                .add_plugins(plugins::camera::Camera)
                .run();
}
