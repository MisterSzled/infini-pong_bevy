use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod events;

fn main() {
        App::new()
                .add_plugins(DefaultPlugins)

                .insert_resource(ClearColor(Color::rgb(1.0, 0., 0.)))

                .init_resource::<resources::player::player_movement::PlayerAvailableMovement>()

                .add_event::<events::movement::movement_request::MovementRequest>()

                .add_plugins(plugins::camera::Camera)
                .add_plugins(plugins::player::Player)
                .add_plugins(plugins::input::Input)
                .add_plugins(plugins::wall::Wall)
                .run();
}