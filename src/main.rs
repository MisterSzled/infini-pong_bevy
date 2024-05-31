use bevy::prelude::*;

mod components;
mod events;
mod plugins;
mod resources;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppStates {
        MainMenu,
        InGame,
}

fn main() {
        App::new()
                .insert_state(AppStates::MainMenu)

                .add_plugins(DefaultPlugins)

                .init_resource::<resources::player::player_movement::PlayerAvailableMovement>()

                .add_event::<events::movement::movement_request::MovementRequest>()

                .add_plugins(plugins::camera::Camera)

                .add_plugins(plugins::player::Player)
                .add_plugins(plugins::enemy::Enemy)
                .add_plugins(plugins::ball::Ball)
                .add_plugins(plugins::input::Input)
                .add_plugins(plugins::wall::Wall)
                .run();
}
