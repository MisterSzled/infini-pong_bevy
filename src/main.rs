use bevy::prelude::*;

mod components;
mod events;
mod plugins;
mod resources;
mod materials;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppStates {
        #[default]
        Setup,
        InGame,
        MainMenu,
}

fn main() {
        App::new()
                .add_plugins(DefaultPlugins.set(
                        ImagePlugin::default_nearest()
                ))

                .init_state::<AppStates>()

                .init_resource::<resources::config::config::Config>()
                .add_systems(Startup, resources::textures::loader::load)
                .add_systems(Startup, resources::levels::loader::load)
                .add_plugins(plugins::state_manager::StateManager)

                .add_plugins(plugins::camera::Camera)

                .add_plugins(plugins::ui::UI)
                .add_plugins(plugins::match_info::MatchInfo)
                .add_plugins(plugins::player::Player)
                .add_plugins(plugins::enemy::Enemy)
                .add_plugins(plugins::ball::Ball)
                .add_plugins(plugins::input::Input)
                .add_plugins(plugins::wall::Wall)
                .add_plugins(plugins::map::Map)

                .init_resource::<resources::player::player_movement::PlayerAvailableMovement>()
                
                .add_event::<events::movement::movement_request::MovementRequest>()
                .add_event::<events::score::increment_score_event::IncrementScoreEvent>()
                .add_event::<events::score::reset_ball_event::ResetBallEvent>()

                .run();
}

// 16 * 96 => 1536
// 1600 - 1536 => 64