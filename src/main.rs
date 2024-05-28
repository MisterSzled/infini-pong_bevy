use bevy::prelude::*;

mod plugins;

fn main() {
        App::new()
                .add_plugins(DefaultPlugins)
                .insert_resource(ClearColor(Color::rgb(1.0, 0., 0.)))
                .add_plugins(plugins::camera::Camera)
                .add_plugins(plugins::player::Player)
                .run();
}
