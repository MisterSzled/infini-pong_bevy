use crate::AppStates;
use bevy::prelude::*;

mod init;
mod listener;
mod reset_handler;

pub struct MatchInfo;

impl Plugin for MatchInfo {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        init::setup,
                );
                app.add_systems(
                        Update,
                        listener::listen.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
                app.add_systems(
                        Update,
                        reset_handler::reset_handler.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
        }
}
